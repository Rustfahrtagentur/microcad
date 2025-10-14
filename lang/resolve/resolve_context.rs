// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve Context

use crate::{diag::*, rc::*, resolve::*, src_ref::*, syntax::*};

/// Resolve Context
#[derive(Default)]
pub struct ResolveContext {
    /// Symbol table.
    pub(crate) symbol_table: SymbolTable,
    /// Source file cache.
    pub(crate) sources: Sources,
    /// Diagnostic handler.
    pub(crate) diag: DiagHandler,
    /// Unchecked symbols.
    ///
    /// Filled by [check()] with symbols which are not in use in ANY checked code.
    unchecked: Option<Symbols>,
    /// Signals resolve stage.
    mode: ResolveMode,
}

/// Select what {ResolveContext::create()] automatically does.
#[derive(Default, PartialEq, PartialOrd)]
pub enum ResolveMode {
    /// Failed context.
    Failed,
    /// Only load the sources.
    #[default]
    Loaded,
    /// Create symbol table.
    Symbolized,
    /// Resolve symbol table.
    Resolved,
    /// Check symbol table.
    Checked,
}

impl ResolveContext {
    /// Create new context from source file.
    ///
    /// Just reads the syntax and does **not** create any symbols nor resolves anything.
    pub fn new(
        root: Rc<SourceFile>,
        search_paths: &[impl AsRef<std::path::Path>],
        diag: DiagHandler,
    ) -> ResolveResult<Self> {
        Ok(Self {
            sources: Sources::load(root.clone(), search_paths)?,
            diag,
            ..Default::default()
        })
    }

    /// Load resolve and check a source file and referenced files.
    pub fn create(
        root: Rc<SourceFile>,
        search_paths: &[impl AsRef<std::path::Path>],
        builtin: Option<Symbol>,
        diag: DiagHandler,
    ) -> ResolveResult<Self> {
        match Self::create_ex(root, search_paths, builtin, diag, ResolveMode::Checked) {
            Ok(context) => Ok(context),
            Err(err) => {
                // create empty context which might be given to following stages like export.
                let mut context = ResolveContext {
                    mode: ResolveMode::Failed,
                    ..Default::default()
                };
                context.error(&SrcRef(None), err)?;
                Ok(context)
            }
        }
    }

    fn create_ex(
        root: Rc<SourceFile>,
        search_paths: &[impl AsRef<std::path::Path>],
        builtin: Option<Symbol>,
        diag: DiagHandler,
        mode: ResolveMode,
    ) -> ResolveResult<Self> {
        let mut context = Self::new(root, search_paths, diag)?;
        context.symbolize()?;
        log::trace!("Symbolized Context:\n{context:?}");
        if let Some(builtin) = builtin {
            log::trace!("Added builtin library {id}.", id = builtin.id());
            context.symbol_table.add_symbol(builtin)?;
        }
        if matches!(mode, ResolveMode::Resolved | ResolveMode::Checked) {
            context.resolve()?;
            if matches!(mode, ResolveMode::Checked) {
                context.check()?;
            }
        }
        Ok(context)
    }

    #[cfg(test)]
    pub(super) fn test_create(root: Rc<SourceFile>, mode: ResolveMode) -> ResolveResult<Self> {
        Self::create_ex(
            root,
            &[] as &[std::path::PathBuf],
            None,
            Default::default(),
            mode,
        )
    }

    #[cfg(test)]
    pub(super) fn test_add_file(&mut self, file: Rc<SourceFile>) {
        let symbol = file
            .symbolize(Visibility::Private, self)
            .expect("symbolize");
        self.symbol_table
            .add_symbol(symbol)
            .expect("symbolize error");
    }

    pub(crate) fn symbolize(&mut self) -> ResolveResult<()> {
        assert!(matches!(self.mode, ResolveMode::Loaded));
        self.mode = ResolveMode::Failed;

        let named_symbols = self
            .sources
            .clone()
            .iter()
            .map(|source| {
                match (
                    self.sources.generate_name_from_path(&source.filename()),
                    source.symbolize(Visibility::Public, self),
                ) {
                    (Ok(name), Ok(symbol)) => Ok((name, symbol)),
                    (_, Err(err)) | (Err(err), _) => Err(err),
                }
            })
            .collect::<ResolveResult<Vec<_>>>()?;

        for (name, symbol) in named_symbols {
            if let Some(id) = name.single_identifier() {
                self.symbol_table.insert_symbol(id.clone(), symbol)?;
            } else {
                unreachable!("name is not an id")
            }
        }

        self.mode = ResolveMode::Symbolized;

        Ok(())
    }

    pub(super) fn resolve(&mut self) -> ResolveResult<()> {
        assert!(matches!(self.mode, ResolveMode::Symbolized));
        self.mode = ResolveMode::Failed;

        // resolve std as first
        if let Some(std) = self.symbol_table.get(&Identifier::no_ref("std")).cloned() {
            std.resolve(self)?;
        }

        let max_passes = 3;
        let mut passes_needed = 0;
        let mut resolved = false;
        for _ in 0..max_passes {
            self.symbol_table
                .symbols()
                .iter()
                .filter(|child| child.is_resolvable())
                .map(|child| child.resolve(self))
                .collect::<Result<Vec<_>, _>>()?;
            passes_needed += 1;
            if !self.has_links() {
                resolved = true;
                break;
            }
        }

        if resolved {
            log::info!("Resolve OK ({passes_needed} passes).");
        } else {
            log::info!("Resolve failed after {passes_needed} passes.");
        }
        log::debug!("Resolved symbol table:\n{self:?}");

        self.mode = ResolveMode::Resolved;

        Ok(())
    }

    fn has_links(&self) -> bool {
        self.symbol_table
            .symbols()
            .iter_mut()
            .any(|symbol| symbol.has_links())
    }

    /// check names in all symbols
    pub fn check(&mut self) -> ResolveResult<()> {
        log::trace!("Checking symbol table");
        self.mode = ResolveMode::Failed;

        let exclude_ids = self.symbol_table.search_target_mode_ids()?;
        log::trace!("Excluding target mode ids: {exclude_ids}");

        if let Err(err) = self
            .symbol_table
            .symbols()
            .iter_mut()
            .try_for_each(|symbol| symbol.check(self, &exclude_ids))
        {
            self.error(&crate::src_ref::SrcRef::default(), err)?;
        } else if !self.has_errors() {
            self.mode = ResolveMode::Checked;
        }

        log::info!("Symbol table OK!");

        let unchecked = self.symbol_table.unchecked();
        log::trace!(
            "Symbols never used in ANY code:\n{}",
            unchecked
                .iter()
                .map(|symbol| format!("{symbol:?}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        self.unchecked = Some(unchecked);

        Ok(())
    }

    /// Load file into source cache and symbolize it into a symbol.
    pub fn symbolize_file(
        &mut self,
        visibility: Visibility,
        parent_path: impl AsRef<std::path::Path>,
        id: &Identifier,
    ) -> ResolveResult<Symbol> {
        self.sources
            .load_file(parent_path, id)?
            .symbolize(visibility, self)
    }

    /// Create a symbol out of all sources (without resolving them)
    /// Return `true` if context has been resolved (or checked as well)
    pub fn is_checked(&self) -> bool {
        self.mode >= ResolveMode::Checked
    }
}

impl WriteToFile for ResolveContext {}

impl PushDiag for ResolveContext {
    fn push_diag(&mut self, diag: Diagnostic) -> DiagResult<()> {
        self.diag.push_diag(diag)
    }
}

impl Diag for ResolveContext {
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.diag.pretty_print(f, self)
    }

    fn warning_count(&self) -> u32 {
        self.diag.error_count()
    }

    fn error_count(&self) -> u32 {
        self.diag.error_count()
    }

    fn error_lines(&self) -> std::collections::HashSet<usize> {
        self.diag.error_lines()
    }

    fn warning_lines(&self) -> std::collections::HashSet<usize> {
        self.diag.warning_lines()
    }
}

impl GetSourceByHash for ResolveContext {
    fn get_by_hash(&self, hash: u64) -> ResolveResult<std::rc::Rc<SourceFile>> {
        self.sources.get_by_hash(hash)
    }
}

impl std::fmt::Debug for ResolveContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Sources:\n")?;
        write!(f, "{:?}", &self.sources)?;
        writeln!(f, "\nSymbols:\n")?;
        write!(f, "{:?}", &self.symbol_table)?;
        let err_count = self.diag.error_count();
        if err_count == 0 {
            writeln!(f, "No errors.")?;
        } else {
            writeln!(f, "\n{err_count} error(s):\n")?;
            self.diag.pretty_print(f, &self.sources)?;
        }
        if let Some(unchecked) = &self.unchecked {
            writeln!(f, "\nUnchecked:\n{unchecked}")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for ResolveContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(unchecked) = &self.unchecked {
            writeln!(f, "Resolved & checked symbols:\n{}", self.symbol_table)?;
            if unchecked.is_empty() {
                writeln!(f, "All symbols are referenced.\n{}", self.symbol_table)?;
            } else {
                writeln!(
                    f,
                    "Unreferenced symbols:\n{}\n",
                    unchecked
                        .iter()
                        .filter(|symbol| !symbol.is_deleted())
                        .map(|symbol| symbol.full_name().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
            }
        } else {
            writeln!(f, "Resolved symbols:\n{}", self.symbol_table)?;
        }
        if self.has_errors() {
            writeln!(
                f,
                "{err} error(s) and {warn} warning(s) so far:\n{diag}",
                err = self.error_count(),
                warn = self.warning_count(),
                diag = self.diagnosis()
            )?;
        } else {
            writeln!(f, "No errors so far.")?;
        }
        Ok(())
    }
}

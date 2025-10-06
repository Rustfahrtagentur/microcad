// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve Context

use crate::{diag::*, rc::*, resolve::*, syntax::*};

/// Resolve Context
pub struct ResolveContext {
    /// Symbol table.
    pub symbol_table: SymbolTable,
    /// Source file cache.
    pub sources: Sources,
    /// Diagnostic handler.
    pub diag: DiagHandler,
    /// Unchecked symbols.
    ///
    /// Filled by [check()] with symbols which are not in use in ANY checked code.
    pub unchecked: Option<Symbols>,
}

impl ResolveContext {
    /// Create new resolve context.
    pub(super) fn new(sources: Sources, diag: DiagHandler) -> Self {
        Self {
            symbol_table: SymbolTable::default(),
            sources,
            diag,
            unchecked: Default::default(),
        }
    }

    /// Load and resolve a source file
    pub fn load_and_resolve(
        root: Rc<SourceFile>,
        search_paths: &[impl AsRef<std::path::Path>],
        builtin: Option<Symbol>,
        diag: DiagHandler,
    ) -> ResolveResult<ResolveContext> {
        let mut context = Self::load(root, search_paths, diag)?;
        if let Some(builtin) = builtin {
            context.add_symbol(builtin)?;
        }
        context.resolve()?;
        Ok(context)
    }

    pub(crate) fn load(
        root: Rc<SourceFile>,
        search_paths: &[impl AsRef<std::path::Path>],
        diag: DiagHandler,
    ) -> ResolveResult<ResolveContext> {
        let sources = Sources::load(root.clone(), search_paths)?;
        let mut context = Self::new(sources, diag);
        context.symbolize()?;
        Ok(context)
    }

    pub(super) fn add_symbol(&mut self, symbol: Symbol) -> ResolveResult<()> {
        self.symbol_table.add_symbol(symbol.clone())?;
        symbol.resolve(self)?;
        Ok(())
    }

    pub(super) fn resolve(&mut self) -> ResolveResult<()> {
        self.symbol_table
            .values()
            .iter()
            .filter(|child| child.resolvable())
            .try_for_each(|child| {
                child.resolve(self)?;
                Ok::<_, ResolveError>(())
            })?;

        log::debug!("Resolve OK!");
        log::trace!("Resolved symbol table:\n{self:?}");

        Ok(())
    }

    /// check names in all symbols
    pub fn check(&mut self) -> ResolveResult<()> {
        log::trace!("Checking symbol table");
        self.symbol_table
            .values()
            .iter_mut()
            .try_for_each(|symbol| symbol.check(self))?;

        log::debug!("Symbol table OK!");

        let unchecked = self.symbol_table.unchecked();
        log::trace!(
            "Symbols never used in ANY code:\n{}",
            unchecked
                .iter()
                .map(|symbol| format!("{:?}", symbol))
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

    /// Create a symbol out of all sources (without resolving them).
    pub(crate) fn symbolize(&mut self) -> ResolveResult<()> {
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
                todo!()
            }
        }
        Ok(())
    }
}

impl WriteToFile for ResolveContext {}

impl Lookup for ResolveContext {
    fn lookup(&self, name: &QualifiedName) -> Result<Symbol, ResolveError> {
        let symbol = self.symbol_table.lookup(name)?;
        symbol.set_check();
        Ok(symbol)
    }
}

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
                "There were {err} error(s) and {warn} warning(s) so far:\n{diag}",
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

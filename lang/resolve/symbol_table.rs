// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{rc::*, resolve::*, syntax::*};

/// *Symbol table* holding global and local symbols.
///
/// The symbol table consists of the following members:
///
/// - One *root [`Symbol`]* resolved from the *initial source file*.
/// - A [`SourceCache`] of all *loaded source files* (accessible by *qualified name*, *file path* and *source hash*).
/// - A [`Stack`] of [`StackFrame`]s.
/// - A [`SymbolMap`] of all *global symbols*.
///
/// All these internal structures can be accessed by several implemented traits.
#[derive(Default)]
pub struct SymbolTable {
    /// Source file cache.
    pub sources: Sources,
    /// Global symbols (including root).
    symbols: SymbolMap,
    /// Diagnostics handler.
    diag: DiagHandler,
}

impl SymbolTable {
    /// Root symbol (symbol node of initially read source file)
    /// List of all global symbols.
    /// Stack of currently opened scopes with local symbols while evaluation.
    /// Source file cache containing all source files loaded in the context and their syntax trees.
    pub fn new(symbols: SymbolMap, sources: Sources, diag: DiagHandler) -> ResolveResult<Self> {
        // prepare symbol map

        let symbol_table = Self {
            sources,
            symbols,
            diag,
        };
        log::trace!("Initial symbol table:\n{symbol_table}");
        Ok(symbol_table)
    }

    /// Load a symbol table from sources.
    ///
    /// # Arguments
    /// - `root`: root file
    /// - `search_paths`: paths to search for external modules
    ///
    /// Returns a symbol table which is unresolved.
    pub fn load(
        root: Rc<SourceFile>,
        search_paths: &[impl AsRef<std::path::Path>],
        diag: DiagHandler,
    ) -> ResolveResult<Self> {
        // load syntax of root source and external sources
        let sources = Sources::load(root.clone(), search_paths)?;

        let symbols: SymbolMap = sources.symbolize(&diag)?;
        let symbol_table = Self {
            sources,
            symbols,
            diag,
        };
        log::trace!("Initial symbol table:\n{symbol_table}");
        Ok(symbol_table)
    }

    /// Resolve the symbol map.
    pub fn resolve(&mut self) -> ResolveResult<()> {
        todo!()
    }

    pub fn check(&self) -> ResolveResult<()> {
        todo!()
    }

    /// Solve any alias within the given qualified name.
    ///
    /// # Example
    /// ```µcad
    /// mod my {
    ///   use std::geo2d;
    /// }
    /// my::geo2d::rect(1mm);
    /// ```
    pub fn de_alias(&self, name: &QualifiedName) -> QualifiedName {
        (1..name.len())
            .rev()
            .filter_map(|p| {
                if let Ok(symbol) = self.lookup(&QualifiedName::no_ref(name[0..p].to_vec())) {
                    Some((p, symbol))
                } else {
                    None
                }
            })
            .find_map(|(p, symbol)| {
                symbol.with_def(|def| {
                    if let SymbolDefinition::Alias(.., alias) = def {
                        let suffix: QualifiedName = name[p..].iter().cloned().collect();
                        let new_name = suffix.with_prefix(alias);
                        log::trace!("De-aliased name: {name:?} into {new_name:?}");
                        Some(new_name)
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(name.clone())
    }

    pub fn follow_alias(&self, symbol: &Symbol) -> ResolveResult<Symbol> {
        // execute alias from any use statement
        symbol.with_def(|def| {
            if let SymbolDefinition::Alias(.., name) = def {
                log::trace!("{found} alias => {name:?}", found = crate::mark!(FOUND));
                Ok(self.lookup(name)?)
            } else {
                Ok(symbol.clone())
            }
        })
    }

    /// Return search paths of this symbol table.
    pub fn search_paths(&self) -> &Vec<std::path::PathBuf> {
        self.sources.search_paths()
    }

    /// Collect all symbols engaged in that name.
    ///
    /// Example: `what`=`a::b::c` will return the symbols: `a`,`a::b` and `a::b::c`
    pub fn path_to(&self, what: &QualifiedName) -> ResolveResult<Symbols> {
        self.symbols.path_to(what)
    }

    pub fn root(&self) -> Symbol {
        todo!()
    }
}

impl Lookup for SymbolTable {
    /// Lookup a symbol from global symbols.
    fn lookup(&self, name: &QualifiedName) -> ResolveResult<Symbol> {
        log::trace!("Looking for global symbol '{name:?}'");
        let symbol = match self.symbols.search(name) {
            Ok(symbol) => symbol.clone(),
            Err(err) => return Err(err)?,
        };
        log::trace!(
            "{found} global symbol: {symbol}",
            found = crate::mark!(FOUND),
        );
        Ok(symbol)
    }
}

impl std::fmt::Display for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nLoaded files:\n{}", self.sources)?;
        writeln!(f, "\nSymbols:\n{}", self.symbols)
    }
}

impl GetSourceByHash for SymbolTable {
    fn get_by_hash(&self, hash: u64) -> ResolveResult<std::rc::Rc<SourceFile>> {
        self.sources.get_by_hash(hash)
    }
}

impl Diag for SymbolTable {
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.diag.pretty_print(f, self)
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

impl PushDiag for SymbolTable {
    fn push_diag(&mut self, diag: Diagnostic) -> crate::eval::EvalResult<()> {
        self.diag.push_diag(diag)
    }
}

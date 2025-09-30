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
}

impl SymbolTable {
    /// Root symbol (symbol node of initially read source file)
    /// List of all global symbols.
    /// Stack of currently opened scopes with local symbols while evaluation.
    /// Source file cache containing all source files loaded in the context and their syntax trees.
    pub fn new(symbols: SymbolMap, sources: Sources) -> ResolveResult<Self> {
        // prepare symbol map

        let symbol_table = Self { sources, symbols };
        log::trace!("Initial symbol table:\n{symbol_table:?}");
        Ok(symbol_table)
    }

    /// Load a symbol table from sources.
    ///
    /// # Arguments
    /// - `root`: root file
    /// - `search_paths`: paths to search for external modules
    /// - `diag`: Diagnostic handler.
    ///
    /// Returns a symbol table which is unresolved.
    pub(super) fn load(
        root: Rc<SourceFile>,
        search_paths: &[impl AsRef<std::path::Path>],
        diag: DiagHandler,
    ) -> ResolveResult<Self> {
        // load syntax of root source and external sources
        let sources = Sources::load(root.clone(), search_paths)?;
        let context = ResolveContext::new(diag, sources);
        let symbol_table = context.symbolize()?;
        Ok(symbol_table)
    }

    /// Load a symbol table from sources and resolve it.
    ///
    /// # Arguments
    /// - `root`: root file
    /// - `search_paths`: paths to search for external modules
    /// - `builtin`: additional builtin library
    /// - `diag`: Diagnostic handler.
    ///
    /// Returns a symbol table which is unresolved.
    pub fn load_and_resolve(
        root: Rc<SourceFile>,
        search_paths: &[impl AsRef<std::path::Path>],
        builtin: Symbol,
        diag: DiagHandler,
    ) -> ResolveResult<Self> {
        let mut symbol_table = Self::load(root, search_paths, diag)?;
        symbol_table.add_symbol(builtin)?;
        symbol_table.resolve()?;

        Ok(symbol_table)
    }

    /// Resolve the symbol map.
    pub fn resolve(&mut self) -> ResolveResult<()> {
        let children = self.symbols.values().cloned().collect::<Vec<_>>();
        children
            .iter()
            .filter(|child| child.resolvable())
            .try_for_each(|child| {
                child.resolve(self)?;
                Ok::<_, ResolveError>(())
            })?;

        log::trace!("Resolved symbol table:\n{self:?}");

        Ok(())
    }

    pub(super) fn check(&self) -> ResolveResult<()> {
        self.symbols
            .values()
            .try_for_each(|symbol| symbol.check(self))
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

    /// If given symbol is an alias returns the target or the symbol itself if not.
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

    /// Return root symbol (evaluation starting point)
    pub fn main(&self) -> ResolveResult<Symbol> {
        self.lookup(&self.sources.root().name)
    }

    /// Add a new symbol to the table
    pub fn add_symbol(&mut self, symbol: Symbol) -> ResolveResult<()> {
        log::trace!("add symbol: {}", symbol.id());
        if let Some(symbol) = self.symbols.insert(symbol.id(), symbol.clone()) {
            Err(ResolveError::AmbiguousSymbol(symbol.id()))
        } else {
            symbol.resolve(self)?;
            Ok(())
        }
    }

    /// Write symbols into file
    pub fn write_to_file(&self, filename: &impl AsRef<std::path::Path>) -> std::io::Result<()> {
        self.symbols.write_to_file(filename)
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

impl std::fmt::Debug for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nLoaded files:\n{}", self.sources)?;
        writeln!(f, "\nSymbols:\n{:?}", self.symbols)
    }
}

impl GetSourceByHash for SymbolTable {
    fn get_by_hash(&self, hash: u64) -> ResolveResult<std::rc::Rc<SourceFile>> {
        self.sources.get_by_hash(hash)
    }
}

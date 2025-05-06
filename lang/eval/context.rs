// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, eval::*, rc::*, resolve::*, syntax::*};

/// *Context* for *evaluation* of a resolved µcad file.
///
/// The context is used to store the current state of the evaluation.
///
/// A context consists of the following members:
///
/// - A *symbol table* ([`SymbolTable`]) with symbols stored by [`QualifiedName`].
/// - A *diagnostic handler* ([`DiagHandler`]) that accumulates *evaluation errors* for later output.
/// - One *output channel* ([`Output`]) where `__builtin::print` writes it's output to while evaluation.
///
/// All these internal structures can be accessed by several implemented traits.
pub struct Context {
    /// Symbol table
    symbol_table: SymbolTable,
    /// Source file diagnostics-
    diag_handler: DiagHandler,
    /// Output channel for [__builtin::print].
    output: Box<dyn Output>,
}

impl Context {
    /// Create a new context from a source file.
    ///
    /// # Arguments
    /// - `root`: Root symbol
    /// - `builtin`: The builtin library
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library)
    /// - `output`: Output channel to use
    pub fn new(
        root: Symbol,
        builtin: Symbol,
        search_paths: &[std::path::PathBuf],
        output: Box<dyn Output>,
    ) -> Self {
        log::debug!(
            "Creating Context (search paths: {})",
            search_paths
                .iter()
                .map(|p| p.to_string_lossy())
                .collect::<Vec<_>>()
                .join(",")
        );

        // put all together
        Self {
            symbol_table: SymbolTable::new(root, builtin, search_paths),
            diag_handler: Default::default(),
            output,
        }
    }

    /// Create a new context from a source file.
    ///
    /// # Arguments
    /// - `root`: Path to the root file to load
    /// - `builtin`: The builtin library
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library)
    pub fn from_source(
        root: impl AsRef<std::path::Path>,
        builtin: Symbol,
        search_paths: &[std::path::PathBuf],
    ) -> EvalResult<Self> {
        Ok(Self::new(
            SourceFile::load(root)?.resolve(None),
            builtin,
            search_paths,
            Box::new(Stdout),
        ))
    }

    /// Create a new context from a source file and capture output (see [`Self::output`]).
    ///
    /// # Arguments
    /// - `root`: Resolved root source file.
    /// - `builtin`: The builtin library
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library)
    pub fn from_source_captured(
        root: Rc<SourceFile>,
        builtin: Symbol,
        search_paths: &[std::path::PathBuf],
    ) -> Self {
        Self::new(
            root.resolve(None),
            builtin,
            search_paths,
            Box::new(Capture::new()),
        )
    }

    /// Access diagnostic handler.
    pub fn diag_handler(&self) -> &DiagHandler {
        &self.diag_handler
    }

    /// Access captured output.
    pub fn output(&self) -> Option<String> {
        self.output.output()
    }

    /// Print for `__builtin::print`.
    pub fn print(&mut self, what: String) {
        self.output.print(what).expect("could not write to output");
    }

    /// Get source code location of a src referrer.
    pub fn locate(&self, referrer: &impl SrcReferrer) -> EvalResult<String> {
        Ok(format!(
            "{}:{}",
            self.get_by_hash(referrer.src_ref().source_hash())?
                .filename_as_str(),
            referrer.src_ref()
        ))
    }

    /// Evaluate context to a value.
    pub fn eval(&mut self) -> EvalResult<Value> {
        let source_file = match &self.symbol_table.root.borrow().def {
            SymbolDefinition::SourceFile(source_file) => source_file.clone(),
            _ => todo!(),
        };
        source_file.eval(self)
    }

    /// Peek into root node for testing
    #[cfg(test)]
    pub fn root(&self) -> &Symbol {
        &self.symbol_table.root
    }
}

impl Locals for Context {
    fn set_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()> {
        self.symbol_table.set_local_value(id, value)
    }

    fn get_local_value(&mut self, id: &Identifier) -> EvalResult<Value> {
        self.symbol_table.get_local_value(id)
    }

    fn open_call(&mut self, symbol: Symbol, args: CallArgumentList, src_ref: SrcRef) {
        self.symbol_table.open_call(symbol, args, src_ref);
    }

    fn open_source(&mut self, id: Identifier) {
        self.symbol_table.open_source(id);
    }

    fn open_namespace(&mut self, id: Identifier) {
        self.symbol_table.open_namespace(id);
    }

    fn open_body(&mut self) {
        self.symbol_table.open_body();
    }

    fn close(&mut self) {
        self.symbol_table.close();
    }

    fn fetch(&self, id: &Identifier) -> EvalResult<Symbol> {
        self.symbol_table.fetch(id)
    }
}

impl Lookup for Context {
    fn lookup(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        self.symbol_table.lookup(name)
    }
}

impl Diag for Context {
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.diag_handler.pretty_print(f, &self.symbol_table)
    }

    fn error_count(&self) -> u32 {
        self.diag_handler.error_count()
    }
}

impl UseSymbol for Context {
    fn use_symbol(&mut self, name: &QualifiedName, id: Option<Identifier>) -> EvalResult<Symbol> {
        self.symbol_table.use_symbol(name, id)
    }

    fn use_symbols_of(&mut self, name: &QualifiedName) -> EvalResult<Symbol> {
        self.symbol_table.use_symbols_of(name)
    }
}

impl PushDiag for Context {
    fn push_diag(&mut self, diag: Diagnostic) -> EvalResult<()> {
        let result = self.diag_handler.push_diag(diag);
        log::trace!("Context:\n{self}");
        result
    }
}

impl GetSourceByHash for Context {
    fn get_by_hash(&self, hash: u64) -> EvalResult<Rc<SourceFile>> {
        self.symbol_table.get_by_hash(hash)
    }
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_errors() {
            write!(f, "{}\nErrors:\n", self.symbol_table)?;
            self.diag_handler.pretty_print(f, &self.symbol_table)
        } else {
            write!(f, "{}", self.symbol_table)
        }
    }
}

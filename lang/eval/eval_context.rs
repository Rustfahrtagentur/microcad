// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, eval::*, rc::*, resolve::*, syntax::*};

/// Context for evaluation of a resolved µcad file.
///
/// The context is used to store the current state of the evaluation.
///
/// A context is consists of the following structures:
///
/// - One *root symbol* resolved from the initially read source file.
/// - A map of all *global symbols* accessible by fully [`QualifiedName`].
/// - A stack of local scope frames that store *local values* and *local symbol aliases*
///   (e.g. use statements) accessible by [`Identifier`].
/// - A *current namespace* while evaluation.
/// - A map of all *loaded source files* (accessible by name, path and hash).
/// - A diagnostic handler that accumulates *evaluation errors* for later output.
/// - One *output channel* where `__builtin::print` writes it's output to while evaluation.
pub struct EvalContext {
    /// Symbol table
    symbol_table: SymbolTable,
    /// Call stack
    call_stack: CallStack,
    /// Source file diagnostics-
    diag_handler: DiagHandler,
    /// Output channel for [__builtin::print].
    output: Box<dyn Output>,
}

impl EvalContext {
    /// Create a new context from a source file.
    ///
    /// # Arguments
    /// - `root`: Root symbol
    /// - `builtin`: The builtin library
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library)
    /// - `output`: Output channel to use
    pub fn new(
        root: SymbolNodeRcMut,
        builtin: SymbolNodeRcMut,
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
            call_stack: Default::default(),
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
        builtin: SymbolNodeRcMut,
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
        builtin: SymbolNodeRcMut,
        search_paths: &[std::path::PathBuf],
    ) -> Self {
        Self::new(
            root.resolve(None),
            builtin,
            search_paths,
            Box::new(Capture::new()),
        )
    }

    /// Push a call to stack
    pub fn push_call(
        &mut self,
        symbol_node: SymbolNode,
        args: ArgumentMap,
        src_ref: impl SrcReferrer,
    ) {
        self.call_stack.push(symbol_node, args, src_ref)
    }

    /// Pop a call from stack
    pub fn pop_call(&mut self) {
        self.call_stack.pop();
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
    pub fn root(&self) -> &SymbolNodeRcMut {
        &self.symbol_table.root
    }
}

impl Locals for EvalContext {
    fn add_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()> {
        self.symbol_table.add_local_value(id, value)
    }

    fn open_source(&mut self, id: Identifier) {
        self.symbol_table.open_source(id);
    }

    fn open_namespace(&mut self, id: Identifier) {
        self.symbol_table.open_namespace(id);
    }

    fn open_module(&mut self, id: Identifier) {
        self.symbol_table.open_module(id);
    }

    fn open_scope(&mut self) {
        self.symbol_table.open_scope();
    }

    fn close(&mut self) {
        self.symbol_table.close();
    }

    fn fetch(&self, id: &Identifier) -> EvalResult<SymbolNodeRcMut> {
        self.symbol_table.fetch(id)
    }
}

impl Symbols for EvalContext {
    fn lookup(&mut self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        self.symbol_table.lookup(name)
    }
}

impl Diag for EvalContext {
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.diag_handler.pretty_print(f, &self.symbol_table)
    }

    fn error_count(&self) -> u32 {
        self.diag_handler.error_count()
    }
}

impl Calls for EvalContext {
    fn fmt_calls(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.call_stack.pretty_print(f, &self.symbol_table)
    }

    fn push(&mut self, symbol_node: SymbolNode, args: ArgumentMap, src_ref: impl SrcReferrer) {
        self.call_stack.push(symbol_node, args, src_ref);
    }

    fn pop(&mut self) {
        self.call_stack.pop();
    }
}

impl UseSymbol for EvalContext {
    fn use_symbol(
        &mut self,
        name: &QualifiedName,
        id: Option<Identifier>,
    ) -> EvalResult<SymbolNodeRcMut> {
        self.symbol_table.use_symbol(name, id)
    }

    fn use_symbols_of(&mut self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut> {
        self.symbol_table.use_symbols_of(name)
    }
}

impl PushDiag for EvalContext {
    fn push_diag(&mut self, diag: Diagnostic) -> EvalResult<()> {
        let result = self.diag_handler.push_diag(diag);
        log::trace!("Context:\n{self}");
        result
    }
}

impl GetSourceByHash for EvalContext {
    fn get_by_hash(&self, hash: u64) -> EvalResult<Rc<SourceFile>> {
        self.symbol_table.get_by_hash(hash)
    }
}

impl std::fmt::Display for EvalContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.has_errors() {
            write!(f, "{}\nErrors:\n", self.symbol_table)?;
            self.diag_handler.pretty_print(f, &self.symbol_table)
        } else {
            write!(f, "{}", self.symbol_table)
        }
    }
}

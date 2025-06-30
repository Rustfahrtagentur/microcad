// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    diag::*,
    eval::*,
    model_tree::{ExporterRegistry, ModelNode},
    rc::*,
    resolve::*,
    syntax::*,
};

/// *Context* for *evaluation* of a resolved µcad file.
///
/// The context is used to store the current state of the evaluation.
///
/// A context consists of the following members:
/// - A *symbol table* ([`SymbolTable`]) with symbols stored by [`QualifiedName`] and a [`Stack`].
/// - A *diagnostic handler* ([`DiagHandler`]) that accumulates *evaluation errors* for later output.
/// - One *output channel* ([`Output`]) where `__builtin::print` writes it's output to while evaluation.
///
/// All these internal structures can be accessed by several implemented traits.
pub struct Context {
    /// Symbol table
    symbol_table: SymbolTable,
    /// Source file diagnostics.
    diag_handler: DiagHandler,
    /// Output channel for [__builtin::print].
    output: Box<dyn Output>,

    /// Exporter database
    exporters: ExporterRegistry,

    /// Importer registry
    importers: ImporterRegistry,
}

impl Context {
    /// Create a new context from a source file.
    ///
    /// # Arguments
    /// - `root`: Root symbol.
    /// - `builtin`: The builtin library.
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library).
    /// - `output`: Output channel to use.
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
            exporters: ExporterRegistry::default(),
            importers: ImporterRegistry::default(),
        }
    }

    /// Create a new context from a source file.
    ///
    /// # Arguments
    /// - `root`: Path to the root file to load.
    /// - `builtin`: The builtin library.
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library).
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
    /// - `builtin`: The builtin library.
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library).
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

    /// Access captured output.
    pub fn output(&self) -> Option<String> {
        self.output.output()
    }

    /// Print for `__builtin::print`.
    pub fn print(&mut self, what: String) {
        self.output.print(what).expect("could not write to output");
    }

    /// Get the source code location of the given referrer as string (e.g. `/path/to/file.µcad:52:1`).
    pub fn locate(&self, referrer: &impl SrcReferrer) -> EvalResult<String> {
        Ok(format!(
            "{}:{}",
            self.get_by_hash(referrer.src_ref().source_hash())?
                .filename_as_str(),
            referrer.src_ref()
        ))
    }

    /// Get the original source code of the given referrer.
    pub fn source_code(&self, referrer: &impl SrcReferrer) -> EvalResult<String> {
        Ok(referrer
            .src_ref()
            .source_slice(&self.get_by_hash(referrer.src_ref().source_hash())?.source)
            .to_string())
    }

    /// Evaluate context into a value.
    pub fn eval(&mut self) -> EvalResult<ModelNode> {
        let source_file = match &self.symbol_table.root.borrow().def {
            SymbolDefinition::SourceFile(source_file) => source_file.clone(),
            _ => todo!(),
        };
        source_file.eval(self)
    }

    /// Peek into root node for testing.
    pub fn root(&self) -> &Symbol {
        &self.symbol_table.root
    }

    /// Run the closure `f` within the given `stack_frame`.
    pub fn scope<T>(&mut self, stack_frame: StackFrame, f: impl FnOnce(&mut Context) -> T) -> T {
        self.open(stack_frame);
        let result = f(self);
        self.close();
        result
    }

    /// Import a value with parameters from an argument map.
    ///
    /// The argument map contains filename and importer id.
    pub fn import(&mut self, arg_map: &ArgumentMap) -> EvalResult<Value> {
        let filename: String = arg_map.get("filename");
        let id: String = arg_map.get("id");
        if let Some(value) = self.importers.get_cached(filename.clone(), id.clone()) {
            return Ok(value);
        }

        let importer = if id.is_empty() {
            self.importers.by_filename(&filename)
        } else {
            self.importers.by_id(&id.clone().into())
        };

        match importer {
            Ok(importer) => match importer.import(arg_map) {
                Ok(value) => {
                    self.importers.cache(filename, id, value.clone());
                    return Ok(value);
                }
                Err(err) => self.error(arg_map, err)?,
            },

            Err(err) => {
                self.error(arg_map, err)?;
            }
        }

        Ok(Value::None)
    }
}

impl Locals for Context {
    fn set_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()> {
        self.symbol_table.set_local_value(id, value)
    }

    fn get_local_value(&mut self, id: &Identifier) -> EvalResult<Value> {
        self.symbol_table.get_local_value(id)
    }

    fn open(&mut self, frame: StackFrame) {
        self.symbol_table.open(frame);
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
        log::trace!("Error Context:\n{self}");
        result
    }
}

#[cfg(test)]
impl Default for Context {
    fn default() -> Self {
        Context::new(
            Symbol::new_source(SourceFile::load_from_str("").expect("Valid source file")),
            Symbol::new_module("__builtin".into()),
            &[],
            Box::new(Stdout),
        )
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
            writeln!(f, "{}Errors:", self.symbol_table)?;
            self.diag_handler.pretty_print(f, &self.symbol_table)
        } else {
            write!(f, "{}", self.symbol_table)
        }
    }
}

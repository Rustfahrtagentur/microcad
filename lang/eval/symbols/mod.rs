mod local_stack;
mod source_cache;
mod source_file;
mod symbol_table;

pub use local_stack::*;
pub use source_cache::*;
pub use symbol_table::*;

use crate::{eval::*, syntax::*};

/// Trait to handle symbol table
pub trait Symbols {
    /// Lookup for local or global symbol by qualified name.
    ///
    /// - looks in local stack
    /// - looks in symbol map
    /// - follows aliases (use statements)
    /// - detect any ambiguity
    /// - loads external files
    fn lookup(&mut self, name: &QualifiedName) -> EvalResult<SymbolNodeRcMut>;

    /// Add a named local value to current locals.
    ///
    /// TODO: Is this special function really needed?
    fn add_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()>;

    /// Open a new source scope.
    ///
    /// Adds a fresh table for locals to the stack.
    fn open_source(&mut self, id: Identifier);

    /// Open a new namespace which then will be the current namespace in the context.
    fn open_namespace(&mut self, id: Identifier);

    /// Open a new module which then will be the current namespace in the context.
    fn open_module(&mut self, id: Identifier);

    /// Open a new scope.
    ///
    /// Adds a fresh table for locals to the stack.
    fn open_scope(&mut self);

    /// Close current scope.
    ///
    /// Remove any locals in the current scope and close it.
    fn close(&mut self);

    /// Fetch a value from locals.
    fn fetch_value(&self, name: &QualifiedName) -> EvalResult<Value>;
}

use microcad_lang::{eval::{Context, Symbols}, parse::{ModuleDefinition, SourceFile}};

/// Builder for a context
pub struct ContextBuilder {
    context: Context,
}

impl ContextBuilder {
    /// Create a new context builder from a source file
    /// 
    /// - `source_file`: source file to build the context from
    /// 
    /// # Returns
    /// 
    /// A new context builder
    pub fn new(source_file: SourceFile) -> Self {
        Self {
            context: Context::from_source_file(source_file),
        }
    }

    /// Add the standard library to the context
    pub fn with_std(mut self) -> Self {
        self.context.add_module(crate::builtin_module());
        self
    }

    /// Add a module to the context
    pub fn with_module(mut self, module: std::rc::Rc<ModuleDefinition>) -> Self {
        self.context.add_module(module);
        self
    }

    /// Build the context
    pub fn build(self) -> Context {
        self.context
    }
}

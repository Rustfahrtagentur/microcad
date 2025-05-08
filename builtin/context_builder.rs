// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{eval::*, parse::*, rc::*};

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
    pub fn with_builtin(mut self) -> ParseResult<Self> {
        self.context.add(crate::builtin_module()?.into());
        Ok(self)
    }

    /// Add std library to context
    ///
    /// - `search_path`: path to search for the std library, usually the directory containing the std.µcad file
    pub fn with_std(mut self, search_path: impl AsRef<std::path::Path>) -> ParseResult<Self> {
        self = self.with_builtin()?;

        let std_source_file = SourceFile::load(search_path.as_ref().join("std.µcad"))?;
        let context = Self::new(std_source_file.clone()).with_builtin()?.build();
        let namespace = context
            .current_source_file()
            .expect("std library")
            .eval_as_namespace(&mut self.context, "std".into())
            .expect("valid std library");

        self.context.add_source_file(std_source_file);
        self.context.add(Symbol::Namespace(namespace));

        Ok(self)
    }

    /// Add a module to the context
    pub fn with_module(mut self, module: Rc<ModuleDefinition>) -> Self {
        self.context.add(module.into());
        self
    }

    /// Build the context
    pub fn build(self) -> Context {
        self.context
    }
}

// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builder pattern to build up a context

use std::rc::Rc;

use microcad_lang::{builtin::*, eval::*, resolve::*, syntax::*};

/// Context builder.
pub struct ContextBuilder {
    context: Context,
}

impl ContextBuilder {
    /// Create new context.
    pub fn new(symbols: SymbolMap, sources: Sources, output: Box<dyn Output>) -> Self {
        Self {
            context: Context::new(symbols, sources, output),
        }
    }

    /// Create a new context from a source file and capture output (see [`Output`]).
    ///
    /// # Arguments
    /// - `root`: Resolved root source file.
    /// - `builtin`: The builtin library.
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library).
    pub fn from_source_captured(
        root: Rc<SourceFile>,
        search_paths: &[impl AsRef<std::path::Path>],
    ) -> ResolveResult<Self> {
        let sources = Sources::load(root, search_paths)?;
        let mut symbols = sources.resolve()?;
        symbols.add_node(crate::builtin_module());
        Ok(Self::new(symbols, sources, Box::new(Capture::new()))
            .importers(crate::builtin_importers())
            .exporters(crate::builtin_exporters()))
    }

    /// Set importers to context.
    pub fn importers(mut self, importers: ImporterRegistry) -> Self {
        self.context.set_importers(importers);
        self
    }

    /// Set exporters to context.
    pub fn exporters(mut self, exporters: ExporterRegistry) -> Self {
        self.context.set_exporters(exporters);
        self
    }

    /// Build context.
    pub fn build(self) -> Context {
        self.context
    }
}

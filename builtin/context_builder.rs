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
    pub fn new(
        root: Identifier,
        symbols: SymbolMap,
        sources: Sources,
        output: Box<dyn Output>,
    ) -> Self {
        Self {
            context: Context::new(root, symbols, sources, output),
        }
    }

    /// Create a new context from a source file and capture output (see [`Output`]).
    pub fn from_source_captured(
        root: Rc<SourceFile>,
        libs: &[std::path::PathBuf],
    ) -> ResolveResult<Self> {
        let root_id = root.id();
        let sources = Sources::default(); //load(root, search_paths)?;
        let mut symbols = sources.resolve()?;
        symbols.add_node(crate::builtin_module());
        Ok(
            Self::new(root_id, symbols, sources, Box::new(Capture::new()))
                .importers(crate::builtin_importers())
                .exporters(crate::builtin_exporters()),
        )
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

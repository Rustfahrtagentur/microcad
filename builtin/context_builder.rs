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
        root: Symbol,
        builtin: Symbol,
        search_paths: &[std::path::PathBuf],
        output: Box<dyn Output>,
    ) -> Self {
        Self {
            context: Context::new(root, builtin, search_paths, output),
        }
    }

    /// Create a new context from a source file and capture output (see [`Self::output`]).
    ///
    /// # Arguments
    /// - `root`: Resolved root source file.
    /// - `builtin`: The builtin library.
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library).
    pub fn from_source_captured(root: Rc<SourceFile>, search_paths: &[std::path::PathBuf]) -> Self {
        Self::new(
            root.resolve(None),
            crate::builtin_module(),
            search_paths,
            Box::new(Capture::new()),
        )
        .importers(crate::builtin_importers())
        .exporters(crate::builtin_exporters())
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

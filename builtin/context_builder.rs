// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builder pattern to build up a context

use microcad_lang::{builtin::*, eval::*, resolve::Symbol};

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

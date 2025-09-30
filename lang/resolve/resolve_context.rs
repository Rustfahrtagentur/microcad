// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve Context

use crate::{diag::*, resolve::*, syntax::*};

/// Resolve Context
pub struct ResolveContext {
    diag: DiagHandler,
    sources: Sources,
}

impl ResolveContext {
    /// Create new resolve context
    pub fn new(diag: DiagHandler, sources: Sources) -> Self {
        Self { diag, sources }
    }

    /// Load file into source cache and symbolize it into a symbol.
    pub fn symbolize_file(
        &mut self,
        visibility: Visibility,
        parent_path: impl AsRef<std::path::Path>,
        id: &Identifier,
    ) -> ResolveResult<Symbol> {
        self.sources
            .load_file(parent_path, id)?
            .symbolize(visibility, self)
    }

    /// Create a symbol out of all sources (without resolving them).
    pub(crate) fn symbolize(mut self) -> ResolveResult<SymbolTable> {
        let named_symbols = self
            .sources
            .clone()
            .iter()
            .map(|source| {
                match (
                    self.sources.generate_name_from_path(&source.filename()),
                    source.symbolize(Visibility::Public, &mut self),
                ) {
                    (Ok(name), Ok(symbol)) => Ok((name, symbol)),
                    (_, Err(err)) | (Err(err), _) => Err(err),
                }
            })
            .collect::<ResolveResult<Vec<_>>>()?;

        let mut symbols = SymbolMap::default();
        for (name, symbol) in named_symbols {
            if let Some(id) = name.single_identifier() {
                symbols.insert(id.clone(), symbol);
            } else {
                todo!()
            }
        }
        SymbolTable::new(symbols, self.sources)
    }
}

impl PushDiag for ResolveContext {
    fn push_diag(&mut self, diag: Diagnostic) -> DiagResult<()> {
        self.diag.push_diag(diag)
    }
}

impl Diag for ResolveContext {
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.diag.pretty_print(f, self)
    }

    fn error_count(&self) -> u32 {
        self.diag.error_count()
    }

    fn error_lines(&self) -> std::collections::HashSet<usize> {
        self.diag.error_lines()
    }

    fn warning_lines(&self) -> std::collections::HashSet<usize> {
        self.diag.warning_lines()
    }
}

impl GetSourceByHash for ResolveContext {
    fn get_by_hash(&self, hash: u64) -> ResolveResult<std::rc::Rc<SourceFile>> {
        self.sources.get_by_hash(hash)
    }
}

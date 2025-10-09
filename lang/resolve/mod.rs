// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Single symbol resolving
//!
//! After parsing a source file (see [`mod@crate::parse`]) it must be resolved to get a symbol out of it:
//!
//! ```no_run
//! use microcad_lang::{syntax::*, parse::*, resolve::*};
//!
//! let source_file = SourceFile::load("my.µcad").expect("parsing success");
//!
//! let source_symbol = source_file.resolve();
//! ```
//!
//! To "run" the source file (and get the expected output) it must now be evaluated (see [`crate::eval`])  .

mod externals;
mod grant;
mod names;
mod resolve_context;
mod resolve_error;
mod sources;
mod symbol;
mod symbol_table;
mod symbolize;

use crate::{diag::*, syntax::*};
pub use externals::*;
pub use resolve_context::*;
pub use resolve_error::*;
pub use sources::*;
pub use symbol::*;
pub use symbol_table::*;

use grant::*;
use names::*;

/// Trait to handle symbol table.
pub trait Lookup<E: std::error::Error = ResolveError> {
    /// Lookup for local or global symbol by qualified name.
    ///
    /// - looks on *stack*
    /// - looks in *symbol table*
    /// - follows *aliases* (use statements)
    /// - detect any ambiguity
    ///
    /// # Arguments
    /// -`name`: qualified name to search for
    fn lookup(&self, name: &QualifiedName) -> Result<Symbol, E>;
}

/// Trait for items which can be fully qualified.
pub trait FullyQualify {
    /// Get a fully (up to root of symbol map) qualified name.
    fn full_name(&self) -> QualifiedName;
}

#[test]
fn resolve_test() {
    let root =
        SourceFile::load("../examples/my_brick.µcad").expect("loading of root source file failed");
    log::trace!("Root source file:\n{root}");

    ResolveContext::create(
        root,
        &["../lib"],
        None,
        DiagHandler::default(),
        ResolveMode::Resolved,
    )
    .expect("loading of symbol table failed");
}

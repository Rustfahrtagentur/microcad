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
mod symbol_definition;
mod symbol_map;
mod symbol_table;
mod symbolize;

use crate::{diag::*, syntax::*};
pub use externals::*;
pub use resolve_context::*;
pub use resolve_error::*;
pub use sources::*;
pub use symbol::*;
pub use symbol_definition::*;
pub use symbol_map::*;
pub use symbol_table::*;
pub use symbolize::*;

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

    let mut symbol_table = SymbolTable::load(root, &["../lib"], DiagHandler::default())
        .expect("loading of symbol table failed");

    symbol_table.resolve().expect("resolve failed");

    symbol_table.check().expect("check failed");
}

#[test]
fn resolve_source_file() {
    todo!()
    /*  let source_file =
        SourceFile::load_from_str(r#"part A() { part B() {} } "#).expect("Valid source");

    let symbol = source_file.resolve().expect("expecting resolve success");

    // file <no file>
    //  part a
    //   part b
    assert!(symbol.get(&"A".into()).is_some());
    assert!(symbol.get(&"c".into()).is_none());

    assert!(symbol.search(&"A".into()).is_some());
    assert!(symbol.search(&"A::B".into()).is_some());
    assert!(symbol.search(&"A::B::C".into()).is_none());

    // use std::print; // Add symbol "print" to current symbol symbol
    // part M() {
    //      print("test"); // Use symbol symbol from parent
    // }

    log::trace!("Symbol symbol:\n{symbol}");

    let b = symbol.search(&"A::B".into()).expect("cant find symbol");
    assert!(b.search(&"A".into()).is_none());

    //assert!(symbol_node.search_top_down(&["<no file>".into()]).is_some());

    log::trace!("{symbol}");
    */
}

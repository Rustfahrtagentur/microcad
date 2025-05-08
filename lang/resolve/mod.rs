// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Single symbol resolving
//!
//! After parsing a source file (see [`crate::parse`]) it must be resolved to get a symbol out of it:
//!
//! ```
//! use microcad_lang::{syntax::*, parse::*, resolve::*}
//!
//! let source_file = SourceFile::load("my.µcad").expect("parsing success");
//!
//! let source_symbol = source_file.resolve(None);
//! ```
//!
//! To "run" the source file (and get the expected output) it must now be evaluated (see [`crate::eval`])  .

mod symbol;
mod symbol_definition;
mod symbol_map;

pub use symbol::*;
pub use symbol_definition::*;
pub use symbol_map::*;

use crate::syntax::*;

/// Trait for items which can be fully qualified.
pub trait FullyQualify {
    /// Get a fully (up to root of symbol map) qualified name.
    fn full_name(&self) -> QualifiedName;
}

#[test]
fn resolve_source_file() {
    let source_file =
        SourceFile::load_from_str(r#"module a() { module b() {} } "#).expect("Valid source");

    let symbol_node = source_file.resolve(None);

    // file <no file>
    //  module a
    //   module b
    assert!(symbol_node.get(&"a".into()).is_some());
    assert!(symbol_node.get(&"c".into()).is_none());

    assert!(symbol_node.search(&"a".into()).is_some());
    assert!(symbol_node.search(&"a::b".into()).is_some());
    assert!(symbol_node.search(&"a::b::c".into()).is_none());

    // use std::print; // Add symbol "print" to current symbol node
    // module m() {
    //      print("test"); // Use symbol node from parent
    // }

    log::trace!("Symbol node:\n{symbol_node}");

    let b = symbol_node.search(&"a::b".into()).expect("cant find node");
    assert!(b.search(&"a".into()).is_none());

    //assert!(symbol_node.search_top_down(&["<no file>".into()]).is_some());

    log::trace!("{symbol_node}");
}

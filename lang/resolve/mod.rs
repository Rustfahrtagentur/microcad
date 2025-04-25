// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! external file loading and symbol tree resolving

mod symbol_definition;
mod symbol_map;
mod symbol_node;

pub use symbol_definition::*;
pub use symbol_map::*;
pub use symbol_node::*;

use crate::syntax::*;

/// Trait for items which can be fully qualified
pub trait FullyQualify {
    /// get a fully (up to root of symbol map) qualified name
    fn full_name(&self) -> Option<QualifiedName>;
}

#[test]
fn resolve_source_file() {
    use std::rc::Rc;

    crate::env_logger_init();

    let source_file = Rc::new(
        SourceFile::load_from_str(r#"module a() { module b() {} } "#).expect("Valid source"),
    );

    let symbol_node = source_file.resolve(None);

    // file <no file>
    //  module a
    //   module b
    let symbol_node = symbol_node.borrow();
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
    assert!(b.borrow().search(&"a".into()).is_none());

    //assert!(symbol_node.search_top_down(&["<no file>".into()]).is_some());

    println!("{symbol_node}");
}

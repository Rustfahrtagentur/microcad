// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad symbol tree resolve

mod symbol_definition;
mod symbol_node;
mod symbol_path;

pub use symbol_definition::*;
pub use symbol_node::*;
pub use symbol_path::*;

/// Source File `foo.µcad`
///
/// module a() {
///     b = 42.0;
///     function bar() { 13 }
/// }
/// namespace c { function d() { 23 } }
///
/// Symbol Tree example:
/// foo.µcad
///     ModuleDefinition(a)
///         FunctionDefinition(bar)
///         Statements
///             b
///     NamespaceDefinition(c)
///         d
///
/// Usage:
///
/// foo = a();
/// print("{foo.b}"); // 42.0
///
/// v = c::d();
use crate::{Rc, RcMut, syntax::*};

/// Trait which resolves to SymbolNode reference
pub trait Resolve {
    /// Resolve self into SymbolNode reference
    fn resolve(&self, parent: Option<RcMut<SymbolNode>>) -> RcMut<SymbolNode>;
}

impl Resolve for Rc<ModuleDefinition> {
    fn resolve(&self, parent: Option<RcMut<SymbolNode>>) -> RcMut<SymbolNode> {
        let node = SymbolNode::new(SymbolDefinition::Module(self.clone()), parent);
        node.borrow_mut().children = self.body.fetch_symbol_map(Some(node.clone()));
        node
    }
}

impl Resolve for Rc<NamespaceDefinition> {
    fn resolve(&self, parent: Option<RcMut<SymbolNode>>) -> RcMut<SymbolNode> {
        let node = SymbolNode::new(SymbolDefinition::Namespace(self.clone()), parent);
        node.borrow_mut().children = self.body.fetch_symbol_map(Some(node.clone()));
        node
    }
}

impl Resolve for Rc<FunctionDefinition> {
    fn resolve(&self, parent: Option<RcMut<SymbolNode>>) -> RcMut<SymbolNode> {
        let node = SymbolNode::new(SymbolDefinition::Function(self.clone()), parent);
        node.borrow_mut().children = self.body.fetch_symbol_map(Some(node.clone()));
        node
    }
}

impl Resolve for SymbolDefinition {
    fn resolve(&self, parent: Option<RcMut<SymbolNode>>) -> RcMut<SymbolNode> {
        match &self {
            Self::Module(m) => m.resolve(parent),
            Self::Namespace(n) => n.resolve(parent),
            Self::Function(f) => f.resolve(parent),
            Self::SourceFile(s) => s.resolve(parent),
            // A builtin symbols cannot have child symbols,
            // hence the resolve trait does not need to be implemented
            Self::BuiltinFunction(f) => {
                SymbolNode::new(SymbolDefinition::BuiltinFunction(f.clone()), parent)
            }
            Self::BuiltinModule(m) => {
                SymbolNode::new(SymbolDefinition::BuiltinModule(m.clone()), parent)
            }
        }
    }
}

impl Resolve for Rc<SourceFile> {
    fn resolve(&self, parent: Option<RcMut<SymbolNode>>) -> RcMut<SymbolNode> {
        let node = SymbolNode::new(SymbolDefinition::SourceFile(self.clone()), parent);
        node.borrow_mut().children = Body::fetch_symbol_map_from(&self.body, Some(node.clone()));
        node
    }
}

impl Resolve for SourceFile {
    fn resolve(&self, parent: Option<RcMut<SymbolNode>>) -> RcMut<SymbolNode> {
        let rc = Rc::new(self.clone());
        rc.resolve(parent)
    }
}

#[test]
fn resolve_source_file() {
    let source_file = Rc::new(
        SourceFile::load_from_str(r#"module a() { module b() {} } "#).expect("Valid source"),
    );

    let symbol_node = source_file.resolve(None);

    // file <no file>
    //  module a
    //   module b
    let symbol_node = symbol_node.borrow();
    assert!(symbol_node.fetch(&"a".into()).is_some());
    assert!(symbol_node.fetch(&"c".into()).is_none());

    assert!(symbol_node.search_down(&"a".into()).is_some());
    assert!(symbol_node.search_down(&"a::b".into()).is_some());
    assert!(symbol_node.search_down(&"a::b::c".into()).is_none());

    // use std::print; // Add symbol "print" to current symbol node
    // module m() {
    //      print("test"); // Use symbol node from parent
    // }

    let b = symbol_node
        .search_down(&"a::b".into())
        .expect("cant find node");
    assert!(b.borrow().search_up(&"a".into()).is_some());

    //assert!(symbol_node.search_top_down(&["<no file>".into()]).is_some());

    println!("{symbol_node}");
}

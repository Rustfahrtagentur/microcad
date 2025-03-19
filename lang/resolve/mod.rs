// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad symbol tree resolve

use std::{cell::*, ops::Deref};

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
use crate::{parse::*, Id};

/// Symbol definition
#[derive(Debug)]
pub enum SymbolDefinition {
    /// Source file symbol
    SourceFile(std::rc::Rc<SourceFile>),
    /// Namespace symbol
    Namespace(std::rc::Rc<NamespaceDefinition>),
    /// Module symbol
    Module(std::rc::Rc<ModuleDefinition>),
    /// Function symbol
    Function(std::rc::Rc<FunctionDefinition>),
}

impl SymbolDefinition {
    fn id(&self) -> Id {
        match &self {
            Self::Namespace(n) => n.name.id().clone(),
            Self::Module(m) => m.name.id().clone(),
            Self::Function(f) => f.name.id().clone(),
            Self::SourceFile(s) => s.filename_as_str().into(),
        }
    }
}

impl std::fmt::Display for SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = self.id();
        match self {
            Self::Module(_) => write!(f, "module {}", id),
            Self::Namespace(_) => write!(f, "namespace {}", id),
            Self::Function(_) => write!(f, "function {}", id),
            Self::SourceFile(_) => write!(f, "file {}", id),
        }
    }
}

/// Qualified name of a symbol
pub struct SymbolPath(Vec<Id>);

impl SymbolPath {
    fn pop_top(&self) -> Self {
        Self(self.0[1..].to_vec())
    }
}

impl From<QualifiedName> for SymbolPath {
    fn from(name: QualifiedName) -> Self {
        Self(name.0.iter().map(|n| n.id().clone()).collect::<Vec<_>>())
    }
}

#[cfg(test)]
impl From<&str> for SymbolPath {
    fn from(name: &str) -> Self {
        Self(name.split("::").map(|n| Id::new(n)).collect::<Vec<_>>())
    }
}

impl Deref for SymbolPath {
    type Target = Vec<Id>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Map Id to SymbolNode reference
pub type SymbolMap = std::collections::btree_map::BTreeMap<Id, SymbolNodeRc>;

/// Symbol node
#[derive(Debug)]
pub struct SymbolNode {
    /// Symbol definition
    pub def: SymbolDefinition,
    /// Symbol's parent node
    pub parent: Option<SymbolNodeRc>,
    /// Symbol's children nodes
    pub children: SymbolMap,
}

impl SymbolNode {
    /// Create new reference counted symbol node
    pub fn new(def: SymbolDefinition, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        SymbolNodeRc::new(RefCell::new(SymbolNode {
            def,
            parent,
            children: Default::default(),
        }))
    }

    /// Print out symbols from that point
    pub fn print_symbol(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}{}", "", self.def)?;
        self.children
            .iter()
            .try_for_each(|(_, child)| child.borrow().print_symbol(f, depth + 1))
    }

    /// Insert child and change parent of child to new parent
    pub fn insert_child(parent: SymbolNodeRc, child: SymbolNodeRc) {
        child.borrow_mut().parent = Some(parent.clone());
        let id = child.borrow().def.id();
        parent.borrow_mut().children.insert(id, child);
    }

    /// Fetch child node with an id
    pub fn fetch(&self, id: &Id) -> Option<&SymbolNodeRc> {
        self.children.get(id)
    }

    /// Search in symbol tree by a path, e.g. a::b::c
    pub fn search_top_down(&self, path: &SymbolPath) -> Option<SymbolNodeRc> {
        if let Some(first) = path.first() {
            if let Some(child) = self.fetch(first) {
                let path = &path.pop_top();
                if path.is_empty() {
                    Some(child.clone())
                } else {
                    child.borrow().search_top_down(path)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Search for first symbol in parents
    pub fn search_bottom_up(&self, path: &SymbolPath) -> Option<SymbolNodeRc> {
        if let Some(parent) = &self.parent {
            if let Some(child) = parent.borrow().search_top_down(path) {
                Some(child.clone())
            } else {
                parent.borrow().search_bottom_up(path)
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for SymbolNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_symbol(f, 0)
    }
}

/// Reference to SymbolNode
pub type SymbolNodeRc = std::rc::Rc<std::cell::RefCell<SymbolNode>>;

/// Trait which resolves to SymbolNode reference
pub trait Resolve {
    /// Resolve self into SymbolNode reference
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc;
}

impl Resolve for std::rc::Rc<ModuleDefinition> {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        let node = SymbolNode::new(SymbolDefinition::Module(self.clone()), parent);
        node.borrow_mut().children = self.body.fetch_symbol_map(Some(node.clone()));
        node
    }
}

impl Resolve for std::rc::Rc<NamespaceDefinition> {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        let node = SymbolNode::new(SymbolDefinition::Namespace(self.clone()), parent);
        node.borrow_mut().children = self.body.fetch_symbol_map(Some(node.clone()));
        node
    }
}

impl Resolve for std::rc::Rc<FunctionDefinition> {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        let node = SymbolNode::new(SymbolDefinition::Function(self.clone()), parent);
        node.borrow_mut().children = self.body.fetch_symbol_map(Some(node.clone()));
        node
    }
}

impl Resolve for SymbolDefinition {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        match &self {
            Self::Module(m) => m.resolve(parent),
            Self::Namespace(n) => n.resolve(parent),
            Self::Function(f) => f.resolve(parent),
            Self::SourceFile(s) => s.resolve(parent),
        }
    }
}

impl Resolve for std::rc::Rc<SourceFile> {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        let node = SymbolNode::new(SymbolDefinition::SourceFile(self.clone()), parent);
        node.borrow_mut().children = Body::fetch_symbol_map_from(&self.body, Some(node.clone()));
        node
    }
}

impl Resolve for SourceFile {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        let rc = std::rc::Rc::new(self.clone());
        rc.resolve(parent)
    }
}

#[test]
fn resolve_source_file() {
    let source_file = std::rc::Rc::new(
        SourceFile::load_from_str(r#"module a() { module b() {} } "#).expect("Valid source"),
    );

    let symbol_node = source_file.resolve(None);

    // file <no file>
    //  module a
    //   module b
    let symbol_node = symbol_node.borrow();
    assert!(symbol_node.fetch(&"a".into()).is_some());
    assert!(symbol_node.fetch(&"c".into()).is_none());

    assert!(symbol_node.search_top_down(&"a".into()).is_some());
    assert!(symbol_node.search_top_down(&"a::b".into()).is_some());
    assert!(symbol_node.search_top_down(&"a::b::c".into()).is_none());

    // use std::print; // Add symbol "print" to current symbol node
    // module m() {
    //      print("test"); // Use symbol node from parent
    // }

    let b = symbol_node
        .search_top_down(&"a::b".into())
        .expect("cant find node");
    assert!(b.borrow().search_bottom_up(&"a".into()).is_some());

    //assert!(symbol_node.search_top_down(&["<no file>".into()]).is_some());

    println!("{symbol_node}");
}

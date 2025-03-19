// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad symbol tree resolve

use std::cell::{Ref, RefCell};

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

enum SymbolDefinition {
    SourceFile(std::rc::Rc<SourceFile>),
    Namespace(std::rc::Rc<NamespaceDefinition>),
    Module(std::rc::Rc<ModuleDefinition>),
    Function(std::rc::Rc<FunctionDefinition>),
}

struct S<'a>(&'a SymbolNodeRc);

impl std::fmt::Display for S<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.borrow().print_symbol(f, 0)
    }
}

impl SymbolDefinition {
    fn id(&self) -> Id {
        match &self {
            Self::Namespace(n) => n.name.id().clone(),
            Self::Module(m) => m.name.id().clone(),
            _ => unimplemented!(),
        }
    }
}

impl std::fmt::Display for SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Module(m) => write!(f, "module {}", m.name.id()),
            Self::Namespace(ns) => write!(f, "namespace {}", ns.name.id()),
            Self::Function(func) => write!(f, "function {}", func.name.id()),
            Self::SourceFile(s) => write!(f, "file {}", s.filename_as_str()),
        }
    }
}

pub type SymbolMap = std::collections::btree_map::BTreeMap<Id, SymbolNodeRc>;

//pub type SymbolNode = rctree::Node<SymbolNodeInner>;
pub struct SymbolNode {
    def: SymbolDefinition,
    children: SymbolMap,
}

impl SymbolNode {
    fn id(&self) -> Id {
        self.def.id()
    }

    fn print_symbol(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}{}", "", self.def)?;
        self.children
            .iter()
            .try_for_each(|(_, child)| child.borrow().print_symbol(f, depth + 1))
    }

    fn add_child(&mut self, child: SymbolNodeRc) {
        let id = child.borrow().id();
        self.add_child_with_id(id, child);
    }

    fn add_child_with_id(&mut self, id: Id, child: SymbolNodeRc) {
        self.children.insert(id, child);
    }
}

pub type SymbolNodeRc = std::rc::Rc<std::cell::RefCell<SymbolNode>>;

pub trait Resolve {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc;
}

impl Statement {
    fn definition(&self) -> Option<SymbolDefinition> {
        match &self {
            Statement::Namespace(n) => Some(SymbolDefinition::Namespace(n.clone())),
            Statement::Module(m) => Some(SymbolDefinition::Module(m.clone())),
            _ => None,
        }
    }
}

impl Resolve for std::rc::Rc<ModuleDefinition> {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        let node = SymbolNode {
            def: SymbolDefinition::Module(self.clone()),
            children: Default::default(),
        };
        let rc = SymbolNodeRc::new(RefCell::new(node));

        rc.borrow_mut().children = self.body.fetch_symbol_map(Some(rc.clone()));

        if let Some(parent) = parent {
            rc.borrow_mut().add_child_with_id("super".into(), parent);
        }

        rc
    }
}

impl Resolve for std::rc::Rc<NamespaceDefinition> {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        SymbolNodeRc::new(RefCell::new(SymbolNode {
            def: SymbolDefinition::Namespace(self.clone()),
            children: self.body.fetch_symbol_map(parent),
        }))
    }
}

impl Resolve for std::rc::Rc<FunctionDefinition> {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        SymbolNodeRc::new(RefCell::new(SymbolNode {
            def: SymbolDefinition::Function(self.clone()),
            children: self.body.fetch_symbol_map(parent),
        }))
    }
}

impl Resolve for SymbolDefinition {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        match &self {
            Self::Module(m) => m.resolve(parent),
            Self::Namespace(n) => n.resolve(parent),
            _ => unimplemented!(),
        }
    }
}

impl Resolve for std::rc::Rc<SourceFile> {
    fn resolve(&self, parent: Option<SymbolNodeRc>) -> SymbolNodeRc {
        let node = SymbolNode {
            def: SymbolDefinition::SourceFile(self.clone()),
            children: Default::default(),
        };
        let rc = SymbolNodeRc::new(RefCell::new(node));

        rc.borrow_mut().children = Body::fetch_symbol_map_from(&self.body, Some(rc.clone()));

        if let Some(parent) = parent {
            rc.borrow_mut().add_child_with_id("super".into(), parent);
        }
        rc
    }
}

#[test]
fn resolve_source_file() {
    let source_file = std::rc::Rc::new(
        SourceFile::load_from_str(r#"module a() { module b() {} } "#).expect("Valid source"),
    );

    let symbol_node = source_file.resolve(None);
    let node = S(&symbol_node);

    // file <no file>
    //  module a
    //   module b

    println!("{node}");
}

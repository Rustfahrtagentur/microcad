// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::{rc::*, resolve::*, src_ref::*, syntax::*};
use custom_debug::Debug;

/// Kind of a [`WorkbenchDefinition`].
#[derive(Clone, Debug, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum WorkbenchKind {
    /// 3D part
    Part,
    /// 2D sketch
    Sketch,
    /// Operation
    Operation,
}

impl WorkbenchKind {
    /// return kind name
    pub fn as_str(&self) -> &'static str {
        match self {
            WorkbenchKind::Part => "part",
            WorkbenchKind::Sketch => "sketch",
            WorkbenchKind::Operation => "op",
        }
    }
}

impl std::fmt::Display for WorkbenchKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Workbench definition, e.g `sketch`, `part` or `op`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct WorkbenchDefinition {
    /// Documentation.
    pub doc: DocBlock,
    /// Workbench attributes.
    pub attribute_list: AttributeList,
    /// Visibility from outside modules.
    pub visibility: Visibility,
    /// Workbench kind.
    pub kind: WorkbenchKind,
    /// Workbench name.
    pub id: Identifier,
    /// Workbench's building plan.
    pub plan: ParameterList,
    /// Workbench body
    pub body: Body,
    /// Workbench code reference
    pub src_ref: SrcRef,
}

impl WorkbenchDefinition {
    /// Resolve into SymbolNode.
    pub fn resolve(self: &Rc<Self>, parent: Option<Symbol>) -> Symbol {
        let node = Symbol::new(SymbolDefinition::Workbench(self.clone()), parent);
        node.borrow_mut().children = self.body.resolve(Some(node.clone()));
        node
    }
}

impl<'a> Initialized<'a> for WorkbenchDefinition {
    fn statements(&'a self) -> std::slice::Iter<'a, Statement> {
        self.body.statements.iter()
    }
}

impl SrcReferrer for WorkbenchDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for WorkbenchDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{kind} {id}({plan}) {body}",
            kind = self.kind,
            id = self.id,
            plan = self.plan,
            body = self.body
        )
    }
}

impl TreeDisplay for WorkbenchDefinition {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}Workbench ({kind}) '{id}':",
            "",
            kind = self.kind,
            id = self.id
        )?;
        depth.indent();
        self.doc.tree_print(f, depth)?;
        self.plan.tree_print(f, depth)?;
        self.body.tree_print(f, depth)
    }
}

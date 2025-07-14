// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element

use crate::{rc::*, resolve::*, src_ref::*, syntax::*};
use custom_debug::Debug;

/// Kind of a [Workbench]
#[derive(Clone, Debug, Copy, PartialEq)]
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

/// Workbench definition
#[derive(Clone, Debug)]
pub struct WorkbenchDefinition {
    /// Workbench attributes.
    pub attribute_list: AttributeList,
    /// Workbench kind
    pub kind: WorkbenchKind,
    /// Workbench name.
    pub id: Identifier,
    /// Workbench's building plan
    pub plan: ParameterList,
    /// Workbench body
    pub body: Body,
    /// Workbench code reference
    pub src_ref: SrcRef,
}

impl WorkbenchDefinition {
    /// Create new workbench
    pub fn new(
        attribute_list: AttributeList,
        kind: WorkbenchKind,
        id: Identifier,
        plan: ParameterList,
        body: Body,
        src_ref: SrcRef,
    ) -> Rc<Self> {
        Self {
            attribute_list,
            kind,
            id,
            plan,
            body,
            src_ref,
        }
        .into()
    }

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

impl PrintSyntax for WorkbenchDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}Workbench ({kind}) '{id}':",
            "",
            kind = self.kind,
            id = self.id
        )?;
        self.plan.print_syntax(f, depth + Self::INDENT)?;
        self.body.print_syntax(f, depth + Self::INDENT)
    }
}

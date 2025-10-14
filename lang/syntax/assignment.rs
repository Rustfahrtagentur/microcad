// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad assignment syntax element

use crate::{rc::*, src_ref::*, syntax::*, ty::*};

/// Assignment specifying an identifier, type and value
#[derive(Clone)]
pub struct Assignment {
    /// Value's visibility
    pub visibility: Visibility,
    /// Assignee qualifier
    qualifier: Qualifier,
    /// Assignee
    pub id: Identifier,
    /// Type of the assignee
    pub specified_type: Option<TypeAnnotation>,
    /// Value to assign
    pub expression: Rc<Expression>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Assignment {
    /// Create new assignment.
    pub fn new(
        visibility: Visibility,
        qualifier: Qualifier,
        id: Identifier,
        specified_type: Option<TypeAnnotation>,
        expression: Rc<Expression>,
        src_ref: SrcRef,
    ) -> Self {
        Self {
            visibility,
            qualifier,
            id,
            specified_type,
            expression,
            src_ref,
        }
    }

    /// Get qualifier (makes `pub` => `pub const`)
    pub fn qualifier(&self) -> Qualifier {
        match self.visibility {
            Visibility::Private => self.qualifier,
            Visibility::Public => Qualifier::Const,
            Visibility::Deleted => unreachable!(),
        }
    }
}
impl SrcReferrer for Assignment {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(
                f,
                "{vis}{qual}{id}: {ty} = {expr}",
                vis = self.visibility,
                qual = self.qualifier,
                id = self.id,
                ty = t.ty(),
                expr = self.expression
            ),
            None => write!(
                f,
                "{vis}{qual}{id} = {expr}",
                vis = self.visibility,
                qual = self.qualifier,
                id = self.id,
                expr = self.expression
            ),
        }
    }
}

impl std::fmt::Debug for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(
                f,
                "{vis}{qual}{id:?}: {ty:?} = {expr:?}",
                vis = self.visibility,
                qual = self.qualifier,
                id = self.id,
                ty = t.ty(),
                expr = self.expression
            ),
            None => write!(f, "{} = {}", self.id, self.expression),
        }
    }
}

impl TreeDisplay for Assignment {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}Assignment {vis}{qual}{id}:",
            "",
            vis = self.visibility,
            qual = self.qualifier,
            id = self.id
        )?;
        depth.indent();
        if let Some(specified_type) = &self.specified_type {
            specified_type.tree_print(f, depth)?;
        }
        self.expression.tree_print(f, depth)
    }
}

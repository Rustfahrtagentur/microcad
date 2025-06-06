// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Assignment statement syntax elements

use crate::{modeltree::*, src_ref::*, syntax::*};

/// An assignment statement, e.g. `#[aux] s = sphere(3.0mm);`.
#[derive(Clone, Debug)]
pub struct AssignmentStatement {
    /// List of attributes.
    pub attribute_list: AttributeList,
    /// The actual assignment.
    pub assignment: Assignment,
    src_ref: SrcRef,
}

impl SrcReferrer for AssignmentStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl PrintSyntax for AssignmentStatement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}Assignment '{}'", "", self.assignment)
    }
}

impl std::fmt::Display for AssignmentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.attribute_list.is_empty() {
            writeln!(f, "{}", self.attribute_list)?;
        }
        writeln!(f, "{};", self.assignment)
    }
}

use crate::parser::*;

impl Parse for AssignmentStatement {
    fn parse(pair: Pair) -> crate::parse::ParseResult<Self> {
        Ok(Self {
            attribute_list: pair.find(Rule::attribute_list).unwrap_or_default(),
            assignment: pair.find(Rule::assignment).expect("Assignment"),
            src_ref: pair.into(),
        })
    }
}

use crate::eval::*;
use crate::value::*;

impl Eval for AssignmentStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let value = self
            .assignment
            .expression
            .eval_with_attribute_list(&self.attribute_list, context)?;
        context.set_local_value(self.assignment.id.clone(), value)?;
        Ok(Value::None)
    }
}

impl AssignmentStatement {
    /// Try to evaluate the assignment into nodes.
    pub fn try_eval_to_nodes(&self, context: &mut Context) -> EvalResult<ObjectNodes> {
        let value = self
            .assignment
            .expression
            .eval_with_attribute_list(&self.attribute_list, context)?;

        context.set_local_value(self.assignment.id.clone(), value)?;
        Ok(context
            .get_local_value(&self.assignment.id)
            .expect("Local value")
            .fetch_nodes())
    }
}

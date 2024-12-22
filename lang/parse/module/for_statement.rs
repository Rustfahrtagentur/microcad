// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! For statement parser entity

use crate::{eval::*, parse::*, parser::*, src_ref::*, sym::*};

/// For statement
#[derive(Clone, Debug)]
pub struct ForStatement {
    /// Loop variable
    loop_var: Identifier,
    /// Loop expression
    loop_expr: Expression,
    /// For loop body
    body: NodeBody,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for ForStatement {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for ForStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_for_statement);

        let mut pairs = pair.inner();

        Ok(ForStatement {
            loop_var: Identifier::parse(pairs.next().expect("Identifier expected"))?,
            loop_expr: Expression::parse(pairs.next().expect("Expression expected"))?,
            body: NodeBody::parse(pairs.next().expect("Node body expected"))?,
            src_ref: pair.clone().into(),
        })
    }
}

impl Eval for ForStatement {
    type Output = ();

    fn eval(&self, context: &mut EvalContext) -> std::result::Result<Self::Output, EvalError> {
        match self.loop_expr.eval(context)? {
            Value::List(list) => {
                for value in list.iter() {
                    context.add(Symbol::Value(self.loop_var.id().clone(), value.clone()));
                    self.body.eval(context)?;
                }
            }
            value => {
                context
                    .error_with_stack_trace(self, EvalError::ExpectedRangeInForLoop(value.ty()))?;
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "for {} {}", self.loop_var, self.body)
    }
}

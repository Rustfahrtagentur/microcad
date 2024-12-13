// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! For statement parser entity

use crate::{diag::*, eval::*, parse::*, parser::*, src_ref::*};

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
            loop_var: Identifier::parse(pairs.next().unwrap())?,
            loop_expr: Expression::parse(pairs.next().unwrap())?,
            body: NodeBody::parse(pairs.next().unwrap())?,
            src_ref: pair.clone().into(),
        })
    }
}

impl Eval for ForStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> std::result::Result<Self::Output, EvalError> {
        match self.loop_expr.eval(context)? {
            Value::List(list) => {
                for value in list.iter() {
                    context.add(Symbol::Value(self.loop_var.id().unwrap(), value.clone()));
                    self.body.eval(context)?;
                }
            }
            value => {
                context.error(
                    self,
                    Box::new(EvalError::ExpectedRangeInForLoop(value.ty())),
                )?;
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

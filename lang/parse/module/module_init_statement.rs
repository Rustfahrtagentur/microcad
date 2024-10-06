// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module initialization statement parser entities

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::SrcReferrer};

/// Module initialization statement
#[derive(Clone, Debug)]
pub enum ModuleInitStatement {
    /// Use statement
    Use(UseStatement),
    /// Expresson
    Expression(Expression),
    /// Assignment
    Assignment(Assignment),
}

impl SrcReferrer for ModuleInitStatement {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::Use(us) => us.src_ref(),
            Self::Expression(us) => us.src_ref(),
            Self::Assignment(us) => us.src_ref(),
        }
    }
}

impl Parse for ModuleInitStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let first = pair.inner().next().unwrap();
        Ok(match first.as_rule() {
            Rule::use_statement => ModuleInitStatement::Use(UseStatement::parse(first)?),
            Rule::expression => ModuleInitStatement::Expression(Expression::parse(first)?),
            Rule::assignment => ModuleInitStatement::Assignment(Assignment::parse(first)?),
            _ => unreachable!(),
        })
    }
}

impl Eval for ModuleInitStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> std::result::Result<Self::Output, EvalError> {
        match self {
            Self::Use(use_statement) => use_statement.eval(context),
            Self::Expression(expression) => {
                let value = expression.eval(context)?;
                match value {
                    Value::Node(_) => Ok(()),
                    _ => {
                        //use crate::diagnostics::AddDiagnostic;
                        // TODO Expression results should be Option<Value>
                        //context.error(expression, format!("Expected node, got {}", value.ty()));
                        Ok(())
                    }
                }
            }
            Self::Assignment(assignment) => assignment.eval(context),
        }
    }
}

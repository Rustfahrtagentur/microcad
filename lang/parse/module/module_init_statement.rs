// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module initialization statement parser entities

use crate::{errors::*, parse::*, parser::*, src_ref::SrcReferrer};

/// Module initialization statement
#[derive(Clone, Debug)]
pub enum ModuleInitStatement {
    /// Use statement
    Use(UseStatement),
    /// Expresson
    Expression(Expression),
    /// Assignment
    Assignment(Assignment),
    /// Function definition
    FunctionDefinition(std::rc::Rc<FunctionDefinition>),
}

impl SrcReferrer for ModuleInitStatement {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::Use(us) => us.src_ref(),
            Self::Expression(us) => us.src_ref(),
            Self::Assignment(us) => us.src_ref(),
            Self::FunctionDefinition(us) => us.src_ref(),
        }
    }
}

impl Parse for ModuleInitStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let first = pair.clone().into_inner().next().unwrap();
        Ok(match first.as_rule() {
            Rule::use_statement => ModuleInitStatement::Use(UseStatement::parse(first)?),
            Rule::expression => ModuleInitStatement::Expression(Expression::parse(first)?),
            Rule::assignment => ModuleInitStatement::Assignment(Assignment::parse(first)?),
            Rule::function_definition => {
                ModuleInitStatement::FunctionDefinition(std::rc::Rc::<FunctionDefinition>::parse(
                    first,
                )?)
            }
            _ => unreachable!(),
        })
    }
}


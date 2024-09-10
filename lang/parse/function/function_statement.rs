// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function statement parser entity

use crate::{errors::*, parse::*, parser::*, src_ref::*};

/// Function statement
#[derive(Clone, Debug)]
pub enum FunctionStatement {
    /// Assignment statement
    Assignment(Assignment),
    /// Use statement
    Use(UseStatement),
    /// Function definition
    FunctionDefinition(std::rc::Rc<FunctionDefinition>),
    /// Return statement
    Return(Box<Expression>),
    /// If-then-else statement
    If {
        /// Condition
        condition: Expression,
        /// If body
        if_body: FunctionBody,
        /// Else body
        else_body: FunctionBody,
        /// Source code reference
        src_ref: SrcRef,
    },
}

impl SrcReferrer for FunctionStatement {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::Assignment(a) => a.src_ref(),
            Self::Use(u) => u.src_ref(),
            Self::FunctionDefinition(fd) => fd.src_ref(),
            Self::Return(e) => e.src_ref(),
            Self::If {
                condition: _,
                if_body: _,
                else_body: _,
                src_ref,
            } => src_ref.clone(),
        }
    }
}

impl Parse for FunctionStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::function_statement);

        let mut inner = pair.clone().into_inner();
        let first = inner.next().unwrap();
        let s = match first.as_rule() {
            Rule::assignment => Self::Assignment(Assignment::parse(first)?),
            Rule::use_statement => Self::Use(UseStatement::parse(first)?),
            Rule::function_definition => {
                Self::FunctionDefinition(std::rc::Rc::<FunctionDefinition>::parse(first)?)
            }
            Rule::function_return_statement => Self::Return(Box::new(Expression::parse(first)?)),
            Rule::function_if_statement => {
                let mut pairs = first.into_inner();
                let condition = Expression::parse(pairs.next().unwrap())?;
                let if_body = FunctionBody::parse(pairs.next().unwrap())?;

                match pairs.next() {
                    None => Self::If {
                        condition,
                        if_body,
                        else_body: FunctionBody::default(),
                        src_ref: pair.into(),
                    },
                    Some(p) => {
                        let else_body = FunctionBody::parse(p)?;
                        Self::If {
                            condition,
                            if_body,
                            else_body,
                            src_ref: pair.into(),
                        }
                    }
                }
            }
            rule => unreachable!("Unexpected token in function statement: {:?}", rule),
        };

        Ok(s)
    }
}


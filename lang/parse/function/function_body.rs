// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function body parser entity

use super::FunctionStatement;
use crate::{errors::*, parse::*, parser::*, src_ref::*};

/// Function body
#[derive(Clone, Debug, Default)]
pub struct FunctionBody(pub Vec<FunctionStatement>);

impl SrcReferrer for FunctionBody {
    fn src_ref(&self) -> SrcRef {
        SrcRef::from_vec(&self.0)
    }
}

impl Parse for FunctionBody {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::function_body);

        Ok(Self(
            pair.inner()
                .map(|pair| match pair.as_rule() {
                    Rule::function_statement => FunctionStatement::parse(pair),
                    Rule::expression => Ok(FunctionStatement::Return(Expression::parse(pair)?)),
                    rule => unreachable!("Unexpected token in function body: {:?}", rule),
                })
                .collect::<ParseResult<_>>()?,
        ))
    }
}

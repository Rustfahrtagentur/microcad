// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tuple expression

use crate::{parse::*, parser::*, src_ref::*};

/// TODO: maybe CallArgumentList should be `ArgumentList` and get independent of module `call`?
type ArgumentList = CallArgumentList;

/// Tuple expression
#[derive(Clone, Debug, Default)]
pub struct TupleExpression {
    /// List of tuple members
    pub args: ArgumentList,
    /// Common unit
    pub unit: Option<Unit>,
    /// `true` if this is a named tuple
    pub is_named: bool,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for TupleExpression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for TupleExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        let call_argument_list =
            CallArgumentList::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;
        if call_argument_list.is_empty() {
            return Err(ParseError::EmptyTupleExpression);
        }

        // Count number of positional and named arguments
        let named_count: usize = call_argument_list
            .iter()
            .map(|c| if c.name.is_some() { 1 } else { 0 })
            .sum();

        if named_count > 0 && named_count < call_argument_list.len() {
            return Err(ParseError::MixedTupleArguments);
        }

        Ok(TupleExpression {
            is_named: named_count == call_argument_list.len(),
            args: call_argument_list,
            unit: match inner.next() {
                Some(pair) => Some(Unit::parse(pair)?),
                None => None,
            },
            src_ref: pair.clone().into(),
        })
    }
}

impl std::fmt::Display for TupleExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({})",
            self.args
                .iter()
                .map(|arg| if self.is_named {
                    format!(
                        "{} = {}",
                        arg.name.clone().expect(INTERNAL_PARSE_ERROR),
                        arg.value
                    )
                } else {
                    arg.to_string()
                })
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        if let Some(unit) = self.unit {
            write!(f, "{}", unit)?;
        }
        Ok(())
    }
}

impl Syntax for TupleExpression {
    fn print_syntax(&self, _f: &mut std::fmt::Formatter, _depth: usize) -> std::fmt::Result {
        todo!()
    }
}

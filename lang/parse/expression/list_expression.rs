// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List expression

use crate::{parse::*, parser::*, src_ref::*};

/// List expression (expression list maybe with common unit)
#[derive(Default, Clone, Debug)]
pub struct ListExpression {
    list: ExpressionList,
    unit: Option<Unit>,
    src_ref: SrcRef,
}

impl std::ops::Deref for ListExpression {
    type Target = ExpressionList;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl std::ops::DerefMut for ListExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}

impl SrcReferrer for ListExpression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for ListExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        Ok(Self {
            list: ExpressionList::parse(inner.next().expect("list_expression expected"))?,
            unit: match inner.next() {
                Some(pair) => Some(Unit::parse(pair)?),
                None => None,
            },
            src_ref: pair.clone().into(),
        })
    }
}

impl std::fmt::Display for ListExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{}]{}",
            self.list
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            if let Some(unit) = self.unit {
                unit.to_string()
            } else {
                String::new()
            }
        )
    }
}

impl PrintSyntax for ListExpression {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        if let Some(unit) = self.unit {
            writeln!(f, "{:depth$}ListExpression {unit}:", "")?
        } else {
            writeln!(f, "{:depth$}ListExpression:", "")?
        }
        self.list
            .iter()
            .try_for_each(|e| e.print_syntax(f, depth + 1))
    }
}

// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List of expression

use crate::{src_ref::*, syntax::*};

/// List expression (expression list maybe with common unit)
#[derive(Default, Clone, Debug)]
pub struct ListExpression {
    /// Expression list
    pub list: ExpressionList,
    /// Optional unit
    pub unit: Option<Unit>,
    /// Source code reference
    pub src_ref: SrcRef,
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

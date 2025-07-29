// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List of expression

use crate::{src_ref::*, syntax::*};
use derive_more::{Deref, DerefMut};

/// List expression (expression list maybe with common unit)
#[derive(Default, Clone, Debug, Deref, DerefMut)]
pub struct ArrayExpression {
    /// Expression list
    #[deref]
    #[deref_mut]
    pub list: ExpressionList,
    /// Unit
    pub unit: Unit,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for ArrayExpression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{}]{}",
            self.list
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.unit
        )
    }
}

impl PrintSyntax for ArrayExpression {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        if !matches!(self.unit, Unit::None) {
            writeln!(f, "{:depth$}ListExpression {unit}:", "", unit = self.unit)?
        } else {
            writeln!(f, "{:depth$}ListExpression:", "")?
        }
        self.list
            .iter()
            .try_for_each(|e| e.print_syntax(f, depth + Self::INDENT))
    }
}

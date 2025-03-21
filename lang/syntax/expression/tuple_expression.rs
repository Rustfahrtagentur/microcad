// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tuple expression

use crate::{src_ref::*, syntax::*};

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
    pub src_ref: SrcRef,
}

impl SrcReferrer for TupleExpression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
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
                    if let Some(name) = &arg.name {
                        format!("{} = {}", &name, arg.value)
                    } else {
                        format!("{}", arg.value)
                    }
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

impl PrintSyntax for TupleExpression {
    fn print_syntax(&self, _f: &mut std::fmt::Formatter, _depth: usize) -> std::fmt::Result {
        todo!()
    }
}

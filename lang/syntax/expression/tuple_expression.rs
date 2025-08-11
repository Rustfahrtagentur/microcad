// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tuple expression.

use crate::{src_ref::*, syntax::*};

/// Tuple expression, e.g. `(x=1+2,4,z=9)`.
#[derive(
    Clone, Debug, Default, bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize,
)]
pub struct TupleExpression {
    /// List of tuple members.
    pub args: ArgumentList,
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
                .map(|arg| if let Some(name) = &arg.id {
                    format!("{} = {}", &name, arg.value)
                } else {
                    arg.to_string()
                })
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        Ok(())
    }
}

impl TreeDisplay for TupleExpression {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}TupleExpression:", "")?;
        depth.indent();
        self.args.tree_print(f, depth)
    }
}

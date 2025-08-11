// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax elements related to calls.

mod argument;
mod argument_list;
mod method_call;

pub use argument::*;
pub use argument_list::*;
pub use method_call::*;

use crate::{model::*, src_ref::*, syntax::*, value::*};

/// Call of a *workbench* or *function*.
#[derive(
    Clone, Debug, Default, bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize,
)]
pub struct Call {
    /// Qualified name of the call.
    pub name: QualifiedName,
    /// Argument list of the call.
    pub argument_list: ArgumentList,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl SrcReferrer for Call {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.argument_list)
    }
}

impl TreeDisplay for Call {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}Call '{}':", "", self.name)?;
        depth.indent();
        self.argument_list
            .iter()
            .try_for_each(|a| a.tree_print(f, depth))
    }
}

/// Result of a call.
pub enum CallResult {
    /// Call returned models.
    Models(Vec<Model>),

    /// Call returned a single value.
    Value(Value),

    /// Call returned nothing.
    None,
}

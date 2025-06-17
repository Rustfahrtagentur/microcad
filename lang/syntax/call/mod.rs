// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Syntax elements related to calls.

mod argument;
mod argument_list;
mod method_call;

pub use argument::*;
pub use argument_list::*;
pub use method_call::*;

use crate::{model_tree::*, src_ref::*, syntax::*, value::*};

/// Call of a function or init.
#[derive(Clone, Debug, Default)]
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

impl PrintSyntax for Call {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}Call '{}':", "", self.name)?;
        self.argument_list
            .iter()
            .try_for_each(|a| a.print_syntax(f, depth + 1))
    }
}

/// Result of a call.
pub enum CallResult {
    /// Call returned nodes.
    Nodes(Vec<ModelNode>),

    /// Call returned a single value.
    Value(Value),

    /// Call returned nothing.
    None,
}

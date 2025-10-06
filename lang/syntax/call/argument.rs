// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A single argument

use crate::{ord_map::*, src_ref::*, syntax::*};

/// Argument in a [`Call`].
#[derive(Clone)]
pub struct Argument {
    /// Name of the argument
    pub id: Option<Identifier>,
    /// Value of the argument
    pub value: Expression,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Argument {
    /// Returns the name, if self.name is some. If self.name is None, try to extract the name from the expression
    pub fn derived_name(&self) -> Option<Identifier> {
        match &self.id {
            Some(name) => Some(name.clone()),
            None => self.value.single_identifier().cloned(),
        }
    }
}

impl SrcReferrer for Argument {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl OrdMapValue<Identifier> for Argument {
    fn key(&self) -> Option<Identifier> {
        self.id.clone()
    }
}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.id {
            Some(ref id) => write!(f, "{id} = {}", self.value),
            None => write!(f, "{}", self.value),
        }
    }
}

impl std::fmt::Debug for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.id {
            Some(ref id) => write!(f, "{id:?} = {:?}", self.value),
            None => write!(f, "{:?}", self.value),
        }
    }
}

impl TreeDisplay for Argument {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        match self.id {
            Some(ref id) => writeln!(f, "{:depth$}Argument '{id:?}':", "")?,
            None => writeln!(f, "{:depth$}Argument:", "")?,
        };
        depth.indent();
        self.value.tree_print(f, depth)
    }
}

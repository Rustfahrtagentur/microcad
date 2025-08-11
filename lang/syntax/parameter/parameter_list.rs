// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter list syntax element

use crate::{ord_map::*, syntax::*};
use derive_more::{Deref, DerefMut};

/// Parameter list
#[derive(
    Clone,
    Debug,
    Default,
    Deref,
    DerefMut,
    bincode::Encode,
    bincode::Decode,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct ParameterList(OrdMap<Identifier, Parameter>);

impl ParameterList {
    /// Create new *parameter list* from a map of [`Parameter`]s.
    pub fn new(value: OrdMap<Identifier, Parameter>) -> Self {
        Self(value)
    }

    /// Return ids of all parameters
    pub fn ids(&self) -> impl Iterator<Item = Identifier> {
        self.keys().cloned()
    }

    /// Return if given identifier is in parameter list
    pub fn contains_key(&self, id: &Identifier) -> bool {
        self.iter().any(|p| *id == p.id)
    }
}

impl std::fmt::Display for ParameterList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl TreeDisplay for ParameterList {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}ParameterList:", "")?;
        depth.indent();
        self.0.iter().try_for_each(|p| p.tree_print(f, depth))
    }
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Node marker syntax element

use crate::{src_ref::*, syntax::*};

/// Node marker, e.g. `@children`.
#[derive(Clone, Debug, bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize)]
pub struct Marker {
    /// Marker name, e.g. `children`
    pub id: Identifier,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Marker {
    /// Returns true if the marker is a children marker
    pub fn is_children_marker(&self) -> bool {
        &self.id == "children"
    }
}

impl SrcReferrer for Marker {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.id)
    }
}

impl TreeDisplay for Marker {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}Marker '{}'", "", self.id)
    }
}

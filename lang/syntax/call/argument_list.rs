// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List of arguments syntax entities.

use crate::{ord_map::*, src_ref::*, syntax::*};
use derive_more::{Deref, DerefMut};

/// *Ordered map* of arguments in a [`Call`].
#[derive(Clone, Debug, Default, Deref, DerefMut, serde::Serialize, serde::Deserialize)]
pub struct ArgumentList(pub Refer<OrdMap<Identifier, Argument>>);

impl SrcReferrer for ArgumentList {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for ArgumentList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let mut v = self
                .0
                .value
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>();
            v.sort();
            v.join(", ")
        })
    }
}

impl TreeDisplay for ArgumentList {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}ArgumentList:", "")?;
        depth.indent();
        self.0.value.iter().try_for_each(|p| p.tree_print(f, depth))
    }
}

impl std::ops::Index<&Identifier> for ArgumentList {
    type Output = Argument;

    fn index(&self, name: &Identifier) -> &Self::Output {
        self.0.get(name).expect("key not found")
    }
}

impl std::ops::Index<usize> for ArgumentList {
    type Output = Argument;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0.value[idx]
    }
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List of arguments syntax entities.

use crate::{ord_map::*, src_ref::*, syntax::*};

/// List (ordered map) of arguments.
#[derive(Clone, Debug, Default)]
pub struct ArgumentList(pub Refer<OrdMap<Identifier, Argument>>);

impl SrcReferrer for ArgumentList {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for ArgumentList {
    type Target = OrdMap<Identifier, Argument>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ArgumentList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

impl PrintSyntax for ArgumentList {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ArgumentList:", "")?;
        self.0
            .value
            .iter()
            .try_for_each(|p| p.print_syntax(f, depth + 1))
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

// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List of CallArgument syntax entities

use crate::{ord_map::*, src_ref::*, syntax::*};

/// List of call arguments
#[derive(Clone, Debug, Default)]
pub struct CallArgumentList(pub Refer<OrdMap<Identifier, CallArgument>>);

impl SrcReferrer for CallArgumentList {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for CallArgumentList {
    type Target = OrdMap<Identifier, CallArgument>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CallArgumentList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for CallArgumentList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .value
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl PrintSyntax for CallArgumentList {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}CallArgumentList:", "")?;
        self.0
            .value
            .iter()
            .try_for_each(|p| p.print_syntax(f, depth + 1))
    }
}

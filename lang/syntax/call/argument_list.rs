// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List of arguments syntax entities.

use crate::{src_ref::*, syntax::*};

/// List (ordered map) of arguments.
#[derive(Clone, Debug, Default)]
pub struct ArgumentList(pub Refer<std::collections::HashMap<Identifier, Argument>>);

impl SrcReferrer for ArgumentList {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for ArgumentList {
    type Target = std::collections::HashMap<Identifier, Argument>;

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
        write!(
            f,
            "{}",
            self.0
                .value
                .iter()
                .map(|(id, p)| format!("{id} = {p}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl PrintSyntax for ArgumentList {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ArgumentList:", "")?;
        self.0.value.iter().try_for_each(|(id, p)| {
            write!(f, "{id} = ");
            p.print_syntax(f, depth + 1)
        })
    }
}

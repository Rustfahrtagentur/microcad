// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter list syntax element

use crate::{ord_map::*, syntax::*};

/// Parameter list
#[derive(Clone, Debug, Default)]
pub struct ParameterList(OrdMap<Identifier, Parameter>);

impl ParameterList {
    /// Create new *parameter list* from a map of [`Parameter`]s.
    pub fn new(value: OrdMap<Identifier, Parameter>) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for ParameterList {
    type Target = OrdMap<Identifier, Parameter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ParameterList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

impl PrintSyntax for ParameterList {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ParameterList:", "")?;
        self.0.iter().try_for_each(|p| p.print_syntax(f, depth + 1))
    }
}

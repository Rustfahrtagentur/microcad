// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter list syntax element

use crate::{syntax::*, ty::Type};

/// Parameter list (always sorted by Parameter::id).
#[derive(Clone, Debug, Default)]
pub struct ParameterList(pub Vec<Parameter>);

impl ParameterList {
    /// Create new *parameter list* from a map of [`Parameter`]s.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Insert parameter to list.
    pub fn push(&mut self, parameter: Parameter) {
        self.0.push(parameter);
    }

    /// Add a builtin parameter.
    pub fn add_builtin(mut self, id: &str, ty: Type) -> Self {
        self.push(Parameter::no_ref(id, ty));
        self
    }

    /// Find parameter by id.
    pub fn find(&self, id: &Identifier) -> Option<&Parameter> {
        self.0.iter().find(|parameter| parameter.id == *id)
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

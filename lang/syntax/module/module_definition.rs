// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element

use crate::{src_ref::*, *};

/// Module definition
#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    /// Module name
    pub name: Identifier,
    /// Module parameters (implicit initialization)
    pub parameters: Option<ParameterList>,
    /// Module body
    pub body: Body,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for ModuleDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ModuleDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "module {name}({parameters}) {body}",
            name = self.name,
            parameters = if let Some(parameters) = &self.parameters {
                format!("{parameters}")
            } else {
                "".into()
            },
            body = self.body
        )
    }
}

impl PrintSyntax for ModuleDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ModuleDefinition '{}':", "", self.name)?;
        if let Some(parameters) = &self.parameters {
            parameters.print_syntax(f, depth + 1)?;
        }
        self.body.print_syntax(f, depth + 1)
    }
}

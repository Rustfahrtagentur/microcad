// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Function signature syntax element

use crate::{src_ref::*, syntax::*, ty::*};

/// Parameters and return type of a function
#[derive(Clone, Debug)]
pub struct FunctionSignature {
    /// Function's parameters
    pub parameters: ParameterList,
    /// Function's return type
    pub return_type: Option<TypeAnnotation>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for FunctionSignature {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl FunctionSignature {
    /// Get parameter by name
    pub fn parameter_by_name(&self, name: &Identifier) -> Option<&Parameter> {
        self.parameters.iter().find(|arg| arg.name == *name)
    }
}

impl PrintSyntax for FunctionSignature {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$} Parameters:", "")?;
        self.parameters.print_syntax(f, depth + 2)?;
        if let Some(return_type) = &self.return_type {
            writeln!(f, "{:depth$} Return:", "")?;
            return_type.print_syntax(f, depth + 2)?;
        };
        Ok(())
    }
}

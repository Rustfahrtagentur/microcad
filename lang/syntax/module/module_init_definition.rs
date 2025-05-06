// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module initialization definition syntax element

use crate::{src_ref::*, syntax::*};

/// Module initialization definition
///
/// Example:
///
/// ```uCAD
/// module a(a: Length) {
///     init(b: Length) { a = 2.0*b; } // The init definition
/// }
/// ```
#[derive(Clone, Debug)]
pub struct ModuleInitDefinition {
    /// Parameter list for this init definition
    pub parameters: ParameterList,
    /// Body if the init definition
    pub body: Body,
    /// Source reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for ModuleInitDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ModuleInitDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "init({parameters}) ", parameters = self.parameters)?;
        write!(f, "{body}", body = self.body)
    }
}

impl PrintSyntax for ModuleInitDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ModuleDefinition:", "")?;
        writeln!(f, "{:depth$} Parameters:", "")?;
        self.parameters.print_syntax(f, depth + 2)?;
        writeln!(f, "{:depth$} Body:", "")?;
        self.body.print_syntax(f, depth + 2)
    }
}

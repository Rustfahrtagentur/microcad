// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Initialization definition syntax element

use crate::{src_ref::*, syntax::*};

/// Initialization definition
///
/// Example:
///
/// ```uCAD
/// part a(a: Length) {
///     init(b: Length) { a = 2.0*b; } // The init definition
/// }
/// ```
#[derive(Clone, Debug)]
pub struct InitDefinition {
    /// Parameter list for this init definition
    pub parameters: ParameterList,
    /// Body if the init definition
    pub body: Body,
    /// Source reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for InitDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for InitDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "init({parameters}) ", parameters = self.parameters)?;
        write!(f, "{body}", body = self.body)
    }
}

impl PrintSyntax for InitDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}PartDefinition:", "")?;
        writeln!(f, "{:depth$} Parameters:", "")?;
        self.parameters.print_syntax(f, depth + 2)?;
        writeln!(f, "{:depth$} Body:", "")?;
        self.body.print_syntax(f, depth + 2)
    }
}

/// Iterator over part's init statements
pub struct Inits<'a>(std::slice::Iter<'a, Statement>);

impl<'a> Inits<'a> {
    /// Create new init for a part
    pub fn new(def: &'a PartDefinition) -> Self {
        Self(def.body.statements.iter())
    }
}

impl<'a> Iterator for Inits<'a> {
    type Item = &'a InitDefinition;

    fn next(&mut self) -> Option<Self::Item> {
        for statement in self.0.by_ref() {
            match statement {
                Statement::Init(init_definition) => {
                    return Some(init_definition);
                }
                _ => continue,
            }
        }

        None
    }
}

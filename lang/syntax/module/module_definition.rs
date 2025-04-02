// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element

use crate::{src_ref::*, syntax::*};

/// Module definition
#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    /// Module name
    pub name: Identifier,
    /// Module parameters (implicit initialization)
    pub parameters: ParameterList,
    /// Module body
    pub body: Body,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl ModuleDefinition {
    pub fn inits(&self) -> Inits {
        Inits::new(self)
    }
}

struct Inits<'a> {
    iter: std::slice::Iter<'a, Statement>,
}

impl<'a> Inits<'a> {
    fn new(def: &'a ModuleDefinition) -> Self {
        Self {
            iter: def.body.statements.iter(),
        }
    }
}

impl<'a> Iterator for Inits<'a> {
    type Item = &'a ModuleInitDefinition;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(statement) = self.iter.next() {
            match statement {
                Statement::ModuleInit(module_init_definition) => {
                    return Some(module_init_definition);
                }
                _ => continue,
            }
        }

        None
    }
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
            parameters = self.parameters,
            body = self.body
        )
    }
}

impl PrintSyntax for ModuleDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ModuleDefinition '{}':", "", self.name)?;
        self.parameters.print_syntax(f, depth + 1)?;
        self.body.print_syntax(f, depth + 1)
    }
}

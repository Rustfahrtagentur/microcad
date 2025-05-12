// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element

use crate::{rc::*, src_ref::*, syntax::*};

/// Module definition
#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    /// Module name
    pub id: Identifier,
    /// Module parameters (implicit initialization)
    pub explicit: Rc<ModuleInitDefinition>,
    /// Module body
    pub body: Body,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl ModuleDefinition {
    /// Return iterator over all initializers
    pub fn init_iter(&self) -> ModuleInitIterator {
        ModuleInitIterator::new(self)
    }
}

/// Iterator over modules init statements
pub struct ModuleInitIterator {
    defs: Vec<Rc<ModuleInitDefinition>>,
    index: usize,
}

impl ModuleInitIterator {
    fn new(def: &ModuleDefinition) -> Self {
        Self {
            defs: std::iter::once(def.explicit.clone())
                .chain(
                    def.body
                        .statements
                        .iter()
                        .filter_map(|statement| match statement {
                            Statement::ModuleInit(def) => Some(def.clone()),
                            _ => None,
                        }),
                )
                .collect(),
            index: 0,
        }
    }
}

impl Iterator for ModuleInitIterator {
    type Item = Rc<ModuleInitDefinition>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.defs.len() {
            None
        } else {
            let next = self.defs[self.index].clone();
            self.index += 1;
            Some(next)
        }
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
            name = self.id,
            parameters = self.explicit,
            body = self.body
        )
    }
}

impl PrintSyntax for ModuleDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ModuleDefinition '{}':", "", self.id)?;
        self.explicit.print_syntax(f, depth + 1)?;
        self.body.print_syntax(f, depth + 1)
    }
}

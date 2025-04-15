// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element

use crate::{eval::*, src_ref::*, syntax::*, value::Value};

/// Module definition
#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    /// Module name
    pub id: Identifier,
    /// Module parameters (implicit initialization)
    pub parameters: ParameterList,
    /// Module body
    pub body: Body,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl ModuleDefinition {
    /// Return iterator over all initializers
    pub fn inits(&self) -> Inits {
        Inits::new(self)
    }
}

impl ModuleInitDefinition {}

impl CallTrait for std::rc::Rc<ModuleDefinition> {
    fn call(&self, _args: &CallArgumentList, _context: &mut EvalContext) -> EvalResult<Value> {
        todo!();
        /*match self.inits().find_map(|init| {
            if let Ok(arg_map) = args.get_matching_arguments(context, &init.parameters) {
                Some((init, arg_map))
            } else {
                None
            }
        }) {
            // We have found a matching initializer
            Some((init, arg_map)) => {
                init.body.statements.iter().for_each(|statement| {});
            }
            // We have not found a matching initializer, test if call arguments match parameter list
            None => {
                let _arg_map = args.get_matching_arguments(context, &self.parameters);
            }
        }*/
    }
}

/// Iterator over modules init statements
pub struct Inits<'a>(std::slice::Iter<'a, Statement>);

impl<'a> Inits<'a> {
    fn new(def: &'a ModuleDefinition) -> Self {
        Self(def.body.statements.iter())
    }
}

impl<'a> Iterator for Inits<'a> {
    type Item = &'a ModuleInitDefinition;

    fn next(&mut self) -> Option<Self::Item> {
        for statement in self.0.by_ref() {
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
            name = self.id,
            parameters = self.parameters,
            body = self.body
        )
    }
}

impl PrintSyntax for ModuleDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ModuleDefinition '{}':", "", self.id)?;
        self.parameters.print_syntax(f, depth + 1)?;
        self.body.print_syntax(f, depth + 1)
    }
}

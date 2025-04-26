// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for UseStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        self.decl.eval(context)
    }
}

impl Eval for UseDeclaration {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match &self {
            UseDeclaration::Use(name) => {
                if let Err(err) = context.use_symbol(None, name) {
                    context.error(name, err)?;
                }
            }
            UseDeclaration::UseAll(_name) => todo!(),
            UseDeclaration::UseAlias(name, alias) => {
                if let Err(err) = context.use_symbol(Some(alias.id().clone()), name) {
                    context.error(name, err)?;
                }
            }
        };
        Ok(Value::None)
    }
}

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
            UseDeclaration::Use(qualified_name, _) => {
                if let Err(err) = context.use_symbol(None, qualified_name) {
                    context.error(qualified_name, err)?;
                }
            }
            UseDeclaration::UseAll(_qualified_name, _) => todo!(),
            UseDeclaration::UseAlias(qualified_name, identifier, _) => {
                if let Err(err) = context.use_symbol(Some(identifier.id().clone()), qualified_name)
                {
                    context.error(qualified_name, err)?;
                }
            }
        };
        Ok(Value::None)
    }
}

// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for UseStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        for decl in &self.decls {
            decl.eval(context)?;
        }
        Ok(Value::None)
    }
}

impl Eval for UseDeclaration {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match &self {
            UseDeclaration::Use(qualified_name, _) => context.use_symbol(qualified_name)?,
            UseDeclaration::UseAll(_qualified_name, _) => todo!(),
            UseDeclaration::UseAlias(_qualified_name, _identifier, _) => todo!(),
        };
        Ok(Value::None)
    }
}

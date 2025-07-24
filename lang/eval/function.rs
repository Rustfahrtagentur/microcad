// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Function call evaluation

use crate::{eval::*, syntax::*, value::*};

impl CallTrait for FunctionDefinition {
    fn call(&self, args: &ArgumentValueList, context: &mut Context) -> EvalResult<Value> {
        match ArgumentMatch::find_multi_match(args, &self.signature.parameters.eval(context)?) {
            Ok(matches) => {
                let mut result: Vec<Value> = Vec::new();
                for args in matches {
                    result.push(context.scope(StackFrame::Function(args.into()), |context| {
                        self.body.statements.eval(context)
                    })?);
                }
                if result.len() == 1 {
                    Ok(result.first().expect("one result item").clone())
                } else {
                    Ok(Value::Array(result.into_iter().collect()))
                }
            }

            Err(err) => {
                context.error(args, err)?;
                Ok(Value::None)
            }
        }
    }
}

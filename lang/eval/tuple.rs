// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for TupleExpression {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let (unnamed, named): (Vec<_>, _) = self
            .args
            .eval(context)?
            .iter()
            .map(|(id, arg)| {
                (
                    id.clone(),
                    match arg.value.clone().bundle_unit(self.unit) {
                        Ok(value) => value.clone(),
                        Err(err) => {
                            context.error(self, err).expect("diag error");
                            Value::None
                        }
                    },
                )
            })
            .partition(|(id, _)| id.is_none());

        Ok(Value::Tuple(
            Tuple {
                named: named.into_iter().collect(),
                unnamed: unnamed.into_iter().map(|(_, v)| (v.ty(), v)).collect(),
                src_ref: todo!(),
            }
            .into(),
        ))
    }
}

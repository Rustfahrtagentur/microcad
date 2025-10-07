// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for TupleExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let (unnamed, named): (Vec<_>, _) = self
            .args
            .eval(context)?
            .iter()
            .map(|(id, arg)| (id.clone(), arg.value.clone()))
            .partition(|(id, _)| id.is_none());

        Ok(Value::Tuple(
            Tuple {
                named: named.into_iter().collect(),
                unnamed: unnamed.into_iter().map(|(_, v)| (v.ty(), v)).collect(),
                src_ref: self.src_ref.clone(),
            }
            .into(),
        ))
    }
}

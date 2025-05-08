// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for TupleExpression {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        if self.is_named {
            let mut values = std::collections::BTreeMap::<Identifier, Value>::new();

            for (key, value) in self.args.eval(context)?.iter().map(|arg| {
                (
                    arg.id.clone().expect("name in named tuple"),
                    arg.clone().value,
                )
            }) {
                values.insert(key, value);
            }

            Ok(Value::NamedTuple(NamedTuple::new(values)))
        } else {
            let values = ValueList::new(
                self.args
                    .eval(context)?
                    .iter()
                    .map(|arg| arg.value.clone())
                    .collect(),
            );

            Ok(Value::UnnamedTuple(UnnamedTuple::new(values)))
        }
    }
}

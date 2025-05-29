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
                // Bundle unit: (x=1,y=2)mm = (x=1mm, y=2mm)
                let value = match value.bundle_unit(self.unit) {
                    Ok(value) => value,
                    Err(err) => {
                        context.error(self, err)?;
                        Value::None
                    }
                };

                values.insert(key, value);
            }

            Ok(Value::NamedTuple(NamedTuple::new(values)))
        } else {
            let values = ValueList::new(
                self.args
                    .eval(context)?
                    .iter()
                    .map(|arg| arg.value.clone().bundle_unit(self.unit).unwrap_or_default())
                    .collect(),
            );

            Ok(Value::UnnamedTuple(UnnamedTuple::new(values)))
        }
    }
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad value related evaluation entities

use crate::{eval::*, syntax::*};

impl Eval<ArgumentValue> for Argument {
    /// Evaluate `Argument` and return `ArgumentValue`
    fn eval(&self, context: &mut EvalContext) -> EvalResult<ArgumentValue> {
        Ok(ArgumentValue::new(
            match self.expression.eval(context) {
                Ok(value) => value,
                Err(err) => {
                    context.error(self, err)?;
                    Value::None
                }
            },
            self.expression.single_identifier().cloned(),
            self.src_ref.clone(),
        ))
    }
}

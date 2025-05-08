// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for FormatExpression {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        Ok(Value::String(format!("{}", self.expression.eval(context)?)))
    }
}

impl Eval for FormatString {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let mut result = String::new();
        for elem in &*self.0 {
            match elem {
                FormatStringInner::String(s) => result += &s.value,
                FormatStringInner::FormatExpression(expr) => match expr.eval(context) {
                    Ok(Value::String(s)) => result += &s,
                    Err(e) => return Err(e),
                    _ => unreachable!("FormatExpression must always evaluate to a string"),
                },
            }
        }
        Ok(Value::String(result))
    }
}

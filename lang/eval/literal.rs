// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for NumberLiteral {
    fn eval(&self, _: &mut EvalContext) -> EvalResult<Value> {
        match self.1.ty() {
            Type::Scalar => Ok(Value::Scalar(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            Type::Angle => Ok(Value::Angle(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            Type::Length => Ok(Value::Length(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            Type::Weight => Ok(Value::Weight(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            Type::Area => Ok(Value::Area(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            Type::Volume => Ok(Value::Volume(Refer::new(
                self.normalized_value(),
                self.src_ref(),
            ))),
            _ => unreachable!(),
        }
    }
}

impl Eval for Literal {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match self {
            Literal::Integer(i) => Ok(Value::Integer(i.clone().map(|i| i))),
            Literal::Number(n) => n.eval(context),
            Literal::Bool(b) => Ok(Value::Bool(b.clone().map(|b| b))),
            Literal::Color(c) => Ok(Value::Color(c.clone().map(|c| c))),
        }
    }
}

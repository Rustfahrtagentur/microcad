// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for ExpressionStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        log::debug!("Evaluating expression statement to value:\n{self}");
        context.grant(self)?;
        self.expression.eval(context)
    }
}

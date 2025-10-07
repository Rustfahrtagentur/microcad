// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for NumberLiteral {
    fn eval(&self, _: &mut EvalContext) -> EvalResult<Value> {
        Ok(self.value())
    }
}

impl Eval for Literal {
    fn eval(&self, _: &mut EvalContext) -> EvalResult<Value> {
        Ok(self.value())
    }
}

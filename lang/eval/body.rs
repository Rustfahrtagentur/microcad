// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for Body {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        context.open_scope();
        let result = Body::evaluate_vec(&self.statements, context);
        context.close();
        result
    }
}

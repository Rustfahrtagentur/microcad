// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for Body {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        context.scope(StackFrame::Body(SymbolMap::default()), |context| {
            Body::evaluate_vec(&self.statements, context)
        })
    }
}

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

impl Body {
    /// Evaluate with given local symbols (e.g. from function Arguments).
    pub fn eval_with_locals(&self, locals: SymbolMap, context: &mut Context) -> EvalResult<Value> {
        context.scope(StackFrame::Body(locals), |context| {
            Body::evaluate_vec(&self.statements, context)
        })
    }
}

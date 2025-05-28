// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Body {
    /// Evaluate body
    pub fn eval(&self, symbols: SymbolMap, context: &mut Context) -> EvalResult<Value> {
        context.scope(StackFrame::Body(symbols), |context| {
            log::trace!("body eval:\n{context}");
            Body::evaluate_vec(&self.statements, context)
        })
    }
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, syntax::*};

impl Eval for SourceFile {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let value = context.scope(
            StackFrame::Source(self.id(), SymbolMap::default()),
            |context| Body::evaluate_vec(&self.body, context),
        );
        log::trace!("Evaluated context:\n{context}");
        value
    }
}

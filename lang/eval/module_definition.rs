// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for ModuleDefinition {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        context.grant(self)?;
        context.scope(
            StackFrame::Module(self.id.clone(), Default::default()),
            |context| {
                // avoid body frame
                self.body.statements.eval(context)
            },
        )
    }
}

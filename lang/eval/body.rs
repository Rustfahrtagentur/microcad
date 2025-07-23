// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

/// Evaluate the body into a single object body model: `{}`.
impl Eval<Model> for Body {
    fn eval(&self, context: &mut Context) -> EvalResult<Model> {
        context.scope(StackFrame::Body(SymbolMap::default()), |context| {
            Ok(ModelBuilder::new_object_body()
                .add_children(self.statements.eval(context)?)?
                .build())
        })
    }
}

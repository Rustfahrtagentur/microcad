// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

/// Evaluate the body into a value.
impl Eval<Value> for Body {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        context.scope(StackFrame::Body(SymbolMap::default()), |context| {
            self.statements.eval(context)
        })
    }
}

/// Evaluate the body into a single group: `{}`.
impl Eval<Option<Model>> for Body {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Option<Model>> {
        context
            .scope(StackFrame::Body(SymbolMap::default()), |context| {
                Ok(ModelBuilder::new(Element::Group, self.src_ref())
                    .add_children(self.statements.eval(context)?)?
                    .attributes(self.statements.eval(context)?)
                    .build())
            })
            .map(Some)
    }
}

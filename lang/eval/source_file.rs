// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*, syntax::*};

impl Eval for std::rc::Rc<SourceFile> {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        context.scope(
            StackFrame::Source(self.id(), SymbolMap::default()),
            |context| self.statements.eval(context),
        )
    }
}

impl Eval<Model> for std::rc::Rc<SourceFile> {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Model> {
        context.scope(
            StackFrame::Source(self.id(), SymbolMap::default()),
            |context| {
                let model = ModelBuilder::new(Element::Group, self.src_ref())
                    .add_children(self.statements.eval(context)?)?
                    .attributes(self.statements.eval(context)?)
                    .build();
                Ok(model)
            },
        )
    }
}

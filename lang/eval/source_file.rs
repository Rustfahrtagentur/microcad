// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model_tree::*, syntax::*};

impl Eval<Model> for std::rc::Rc<SourceFile> {
    fn eval(&self, context: &mut Context) -> EvalResult<Model> {
        context.scope(
            StackFrame::Source(self.id(), SymbolMap::default()),
            |context| {
                let model = ModelBuilder::new_object_body()
                    .add_children(self.statements.eval(context)?)?
                    .build();
                model.borrow_mut().origin.source_file = Some(self.clone());
                Ok(model)
            },
        )
    }
}

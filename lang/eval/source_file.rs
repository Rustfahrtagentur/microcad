// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model_tree::*, syntax::*};

impl Eval<ModelNode> for std::rc::Rc<SourceFile> {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNode> {
        context.scope(
            StackFrame::Source(self.id(), SymbolMap::default()),
            |context| {
                Ok(ModelNodeBuilder::new_object_body()
                    .add_children(self.statements.eval(context)?)?
                    .build()
                    .set_original_source_file(self.clone()))
            },
        )
    }
}

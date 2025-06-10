// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model_tree::*, syntax::*};

impl Eval<ModelNode> for SourceFile {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNode> {
        context.scope(
            StackFrame::Source(self.id(), SymbolMap::default()),
            |context| {
                let nodes: ModelNodes = self.statements.eval(context)?;
                Ok(ModelNode::new_empty_object(self.src_ref()).append_children(nodes))
            },
        )
    }
}

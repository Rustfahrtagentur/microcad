// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model_tree::*};

impl Eval<ModelNode> for Body {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNode> {
        context.scope(StackFrame::Body(SymbolMap::default()), |context| {
            let mut builder = ModelNodeBuilder::new_object_body();

            for statement in self.statements.iter() {
                let value = match statement {
                    Statement::Use(_) => continue, // Use statements have been resolved at this point
                    Statement::Assignment(assignment) => assignment.eval(context)?,
                    Statement::Expression(expression) => expression.eval(context)?,
                    Statement::Marker(marker) => marker.eval(context)?,
                    Statement::If(_) => todo!("if statement not implemented"),
                    statement => {
                        use crate::diag::PushDiag;
                        context.error(
                            self,
                            EvalError::StatementNotSupported(statement.clone().into()),
                        )?;
                        Value::None
                    }
                };
                builder = builder.add_children(value.fetch_nodes())?;
            }

            Ok(builder.build())
        })
    }
}

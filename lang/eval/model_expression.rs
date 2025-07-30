// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

impl Eval<Models> for ModelExpression {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        log::trace!("Evaluating expression:\n{self}");
        let models = match self {
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => match Models::binary_op(lhs.eval(context)?, rhs.eval(context)?, op.as_str()) {
                Err(err) => {
                    context.error(self, err)?;
                    Default::default()
                }
                Ok(model) => [model].into_iter().collect(),
            },
            expr => todo!("{expr:?}"),
        };
        Ok(models)
    }
}

impl Eval<Models> for Nested {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        let mut model_stack = Vec::new();
        for item in self.iter() {
            model_stack.push(item.eval(context)?);
        }
        Ok(Models::from_nested_items(&model_stack))
    }
}

impl Eval<Models> for NestedItem {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        match &self {
            NestedItem::Call(call) => Ok(call.eval(context)?),
            NestedItem::QualifiedName(name) => match &context.lookup(name)?.borrow().def {
                SymbolDefinition::Models(_, models) => Ok(models.clone()),
                _ => unreachable!("Unexpected"),
            },
            NestedItem::Body(body) => Ok(body.eval(context)?),
        }
    }
}

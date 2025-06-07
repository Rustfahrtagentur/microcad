// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model_tree::*};

impl Eval for Assignment {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let value = self.expression.eval(context)?;
        context.set_local_value(self.id.clone(), value)?;
        Ok(Value::None)
    }
}

impl Eval for Marker {
    fn eval(&self, _: &mut Context) -> EvalResult<Value> {
        if self.is_children_marker() {
            Ok(Value::from_single_node(ModelNode::new_element(Refer::new(
                Element::ChildrenPlaceholder,
                self.src_ref(),
            ))))
        } else {
            Ok(Value::None)
        }
    }
}

impl Eval for Statement {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        Ok(match self {
            Self::Use(u) => u.eval(context)?,
            Self::Assignment(a) => a.eval(context)?,
            Self::Expression(e) => e.eval(context)?,
            Self::Marker(m) => m.eval(context)?,
            Self::Part(_) | Self::Function(_) | Self::Module(_) => Value::None,
            statement => todo!("{statement}"),
        })
    }
}

impl Eval<ModelNodes> for StatementList {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNodes> {
        let mut model_nodes = ModelNodes::default();
        for s in self.iter() {
            let value = s.eval(context)?;
            model_nodes.append(&mut value.fetch_nodes());
        }
        Ok(model_nodes)
    }
}

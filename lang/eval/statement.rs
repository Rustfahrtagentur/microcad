// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, objects::*};

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
            Ok(Value::from_single_node(ObjectNode::new_from_content(
                ObjectNodeContent::ChildrenNodeMarker,
            )))
        } else {
            Ok(Value::None)
        }
    }
}

impl Eval for Statement {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        match self {
            Self::Use(u) => u.eval(context)?,
            Self::Assignment(a) => a.eval(context)?,
            Self::Expression(e) => e.eval(context)?,
            Self::Marker(m) => m.eval(context)?,
            Self::Part(_) | Self::Function(_) | Self::Module(_) => Value::None,
            statement => todo!("{statement}"),
        };

        Ok(Value::None)
    }
}

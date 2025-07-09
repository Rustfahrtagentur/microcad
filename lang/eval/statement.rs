// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model_tree::*};

impl Assignment {
    /// Check if the specified type matches the found type.
    pub fn type_check(&self, found: Type) -> EvalResult<()> {
        if let Some(ty) = &self.specified_type {
            if ty.ty() != found {
                return Err(EvalError::TypeMismatch {
                    id: self.id.clone(),
                    expected: ty.ty(),
                    found,
                });
            }
        }

        Ok(())
    }
}

impl Eval for Assignment {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        let value = self.expression.eval(context)?;
        if let Err(err) = self.type_check(value.ty()) {
            context.error(self, err)?;
            return Ok(Value::None);
        }
        context.set_local_value(self.id.clone(), value)?;

        Ok(Value::None)
    }
}

impl Eval for Marker {
    fn eval(&self, _: &mut Context) -> EvalResult<Value> {
        if self.is_children_marker() {
            Ok(ModelNodeBuilder::new_children_placeholder().build().into())
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
            Self::Workbench(_) | Self::Function(_) | Self::Module(_) => Value::None,
            statement => todo!("{statement}"),
        })
    }
}

impl Eval<ModelNodes> for Statement {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNodes> {
        let value: Value = self.eval(context)?;
        let nodes = value.fetch_nodes();
        if nodes.output_type() == ModelNodeOutputType::InvalidMixed {
            context.error(self, EvalError::CannotMixGeometry)?;
        }
        Ok(nodes)
    }
}

impl Eval<ModelNodes> for StatementList {
    fn eval(&self, context: &mut Context) -> EvalResult<ModelNodes> {
        let mut output_type = ModelNodeOutputType::NotDetermined;
        let mut nodes = ModelNodes::default();

        for statement in self.iter() {
            let mut statement_nodes: ModelNodes = statement.eval(context)?;
            let node_output_type = statement_nodes.output_type();
            if output_type == ModelNodeOutputType::NotDetermined {
                output_type = node_output_type;
            } else if node_output_type != output_type {
                context.error(statement, EvalError::CannotMixGeometry)?;
                break;
            }
            nodes.append(&mut statement_nodes);
        }

        Ok(nodes)
    }
}

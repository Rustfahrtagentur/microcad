// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element evaluation
// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

mod assignment_statement;
mod expression_statement;
mod if_statement;
mod marker;
mod return_statement;
mod use_statement;

pub use use_statement::*;

impl Eval for Statement {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        match self {
            Self::Workbench(w) => {
                context.grant(w.as_ref())?;
                Ok(Value::None)
            }
            Self::Module(m) => m.eval(context),
            Self::Function(f) => f.as_ref().eval(context),
            Self::Use(u) => {
                u.eval(context)?;
                Ok(Value::None)
            }
            Self::Assignment(a) => {
                a.eval(context)?;
                Ok(Value::None)
            }
            Self::If(i) => i.eval(context),
            Self::Expression(e) => e.eval(context),
            Self::InnerAttribute(i) => {
                context.grant(i)?;
                Ok(Value::None)
            }
            Self::Init(i) => {
                context.grant(i.as_ref())?;
                Ok(Value::None)
            }
            Self::Return(r) => r.eval(context),
        }
    }
}

impl Eval<Option<Model>> for Statement {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<Model>> {
        let model: Option<Model> = match self {
            Self::Workbench(w) => {
                context.grant(w.as_ref())?;
                None
            }
            Self::Module(_) => None,
            Self::Function(f) => {
                context.grant(f.as_ref())?;
                None
            }
            Self::Init(i) => {
                context.grant(i.as_ref())?;
                None
            }
            Self::Return(r) => {
                context.grant(r)?;
                None
            }
            Self::Use(u) => {
                u.eval(context)?;
                None
            }
            Self::Assignment(a) => {
                a.eval(context)?;
                None
            }
            Self::If(i) => i.eval(context)?,
            Self::Expression(e) => e.eval(context)?,
            Self::InnerAttribute(a) => {
                context.grant(a)?;
                Default::default()
            }
        };

        if let Some(ref model) = model {
            if model.deduce_output_type() == OutputType::InvalidMixed {
                context.error(self, EvalError::CannotMixGeometry)?;
            }
        }

        Ok(model)
    }
}

impl Eval<Value> for StatementList {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        for statement in self.iter() {
            if let Value::Return(result) = statement.eval(context)? {
                return Ok(*result);
            }
        }
        Ok(Value::None)
    }
}

/// Parse inner attributes of a statement list.
impl Eval<Attributes> for StatementList {
    fn eval(&self, context: &mut Context) -> EvalResult<Attributes> {
        let mut attributes = Vec::new();
        for statement in self.iter() {
            if let Statement::InnerAttribute(attribute) = statement {
                attributes.append(&mut attribute.eval(context)?);
            }
        }

        Ok(Attributes(attributes))
    }
}

impl Eval<Models> for StatementList {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        let mut models = Models::default();
        let mut output_type = OutputType::NotDetermined;

        for statement in self.iter() {
            if let Some(model) = statement.eval(context)? {
                output_type = output_type.merge(&model.deduce_output_type());
                if output_type == OutputType::InvalidMixed {
                    context.error(statement, EvalError::CannotMixGeometry)?;
                }
                models.push(model);
            }
        }
        models.deduce_output_type();
        Ok(models)
    }
}

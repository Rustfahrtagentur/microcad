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
            Self::Function(f) => {
                context.grant(f.as_ref())?;
                Ok(Value::None)
            }

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
            Self::Marker(_) => unreachable!(),
            Self::Init(_) => unreachable!(),
            Self::Return(r) => r.eval(context),
        }
    }
}

impl Eval<Models> for Statement {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        let models: Models = match self {
            Self::Workbench(w) => {
                context.grant(w.as_ref())?;
                Default::default()
            }
            Self::Module(m) => m.as_ref().eval(context)?,
            Self::Function(f) => {
                context.grant(f.as_ref())?;
                Default::default()
            }
            Self::Init(i) => {
                context.grant(i.as_ref())?;
                Default::default()
            }

            Self::Return(r) => {
                context.grant(r)?;
                Default::default()
            }
            Self::Use(u) => {
                u.eval(context)?;
                Default::default()
            }
            Self::Assignment(a) => {
                a.eval(context)?;
                Default::default()
            }
            Self::If(i) => {
                let model: Option<Model> = i.eval(context)?;
                model.into()
            }
            Self::Expression(e) => e.eval(context)?,
            Self::Marker(m) => m.eval(context)?,
        };

        if models.deduce_output_type() == OutputType::InvalidMixed {
            context.error(self, EvalError::CannotMixGeometry)?;
        }
        Ok(models)
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

impl Eval<Models> for StatementList {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        let mut models = Models::default();
        let mut output_type = OutputType::NotDetermined;

        for statement in self.iter() {
            let mut statement_models: Models = statement.eval(context)?;
            output_type = output_type.merge(&statement_models.deduce_output_type());
            if output_type == OutputType::InvalidMixed {
                context.error(statement, EvalError::CannotMixGeometry)?;
            }

            models.append(&mut statement_models);
        }
        models.deduce_output_type();
        Ok(models)
    }
}

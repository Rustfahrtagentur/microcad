// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parameter syntax elements

mod parameter_list;

use crate::{diag::PushDiag, eval::{EvalContext, EvalError}, ord_map::*, src_ref::*, syntax::*, ty::*, value::Value};

pub use parameter_list::*;

/// A parameter for a function or module definition
#[derive(Clone, Debug, Default)]
pub struct Parameter {
    /// Name of the parameter
    pub name: Identifier,
    /// Type of the parameter or `None`
    pub specified_type: Option<TypeAnnotation>,
    /// default value of the parameter or `None`
    pub default_value: Option<Expression>,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Parameter {
    /// Create new parameter
    pub fn new(
        name: Identifier,
        specified_type: Option<TypeAnnotation>,
        default_value: Option<Expression>,
        src_ref: SrcRef,
    ) -> Self {
        Self {
            name,
            specified_type,
            default_value,
            src_ref,
        }
    }

    /// Evaluate default value considering specified type
    pub fn eval_default_value(&self, context: &mut EvalContext) -> crate::eval::EvalResult<Value>  {
        use crate::eval::Eval;

        match (&self.specified_type, &self.default_value) {
            (Some(specified_type), Some(default_value)) => {
                let value = default_value.eval(context)?;
                if specified_type.ty() != value.ty() {
                    context.error(self.src_ref.clone(), EvalError::ParameterTypeMismatch { name: self.name.clone(), expected: specified_type.ty(), found: value.ty() })?;
                    Ok(Value::None)
                } else {
                    Ok(value)
                }
            }
            (None, Some(default_value)) => Ok(default_value.eval(context)?),
            _ => Ok(Value::None)
        }


    }
}

impl SrcReferrer for Parameter {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl OrdMapValue<Identifier> for Parameter {
    fn key(&self) -> Option<Identifier> {
        Some(self.name.clone())
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
            (Some(t), Some(v)) => write!(f, "{}: {t} = {v}", self.name),
            (Some(t), None) => write!(f, "{}: {t}", self.name),
            (None, Some(v)) => write!(f, "{} = {v}", self.name),
            _ => Ok(()),
        }
    }
}

impl PrintSyntax for Parameter {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
            (Some(specified_type), Some(default_value)) => writeln!(
                f,
                "{:depth$}Parameter: '{}: {} = {}'",
                "", self.name, specified_type, default_value
            ),
            (Some(specified_type), None) => writeln!(
                f,
                "{:depth$}Parameter: '{}: {}'",
                "", self.name, specified_type
            ),
            (None, Some(default_value)) => writeln!(
                f,
                "{:depth$}Parameter: '{} = {}'",
                "", self.name, default_value
            ),
            _ => unreachable!("impossible parameter declaration"),
        }
    }
}

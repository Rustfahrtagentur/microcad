// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter evaluation entity

use crate::{eval::*, syntax::*, ty::*, value::*};

impl Eval<ParameterValue> for Parameter {
    /// Evaluate [Parameter] into [ParameterValue].
    fn eval(&self, context: &mut Context) -> EvalResult<ParameterValue> {
        match (&self.specified_type, &self.default_value) {
            // Type and value are specified
            (Some(specified_type), Some(default_value)) => {
                let default_value: Value = default_value.eval(context)?;
                if specified_type.ty() != default_value.ty() {
                    context.error(
                        self,
                        EvalError::TypeMismatch {
                            id: self.id.clone(),
                            expected: specified_type.ty(),
                            found: default_value.ty(),
                        },
                    )?;
                    // Return an invalid parameter value in case evaluation failed
                    Ok(ParameterValue::invalid(self.src_ref()))
                } else {
                    Ok(ParameterValue {
                        specified_type: Some(specified_type.ty()),
                        default_value: Some(default_value),
                        src_ref: self.src_ref(),
                    })
                }
            }
            // Only type is specified
            (Some(t), None) => Ok(ParameterValue {
                specified_type: Some(t.ty()),
                src_ref: self.src_ref(),
                ..Default::default()
            }),
            // Only value is specified
            (None, Some(expr)) => {
                let default_value: Value = expr.eval(context)?;

                Ok(ParameterValue {
                    specified_type: Some(default_value.ty().clone()),
                    default_value: Some(default_value),
                    src_ref: self.src_ref(),
                })
            }
            // Neither type nor value is specified
            (None, None) => Ok(ParameterValue::invalid(self.src_ref())),
        }
    }
}

impl Eval<ParameterValueList> for ParameterList {
    /// Evaluate [ParameterList] into [ParameterValueList].
    fn eval(&self, context: &mut Context) -> EvalResult<ParameterValueList> {
        let mut values = ParameterValueList::default();
        for parameter in self.iter() {
            values.insert(parameter.id.clone(), parameter.eval(context)?)?;
        }

        Ok(values)
    }
}

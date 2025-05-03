// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter evaluation entity

use crate::{eval::*, syntax::*, ty::*, value::*};

impl ParameterValue {
    fn from_parameter(parameter: &Parameter, context: &mut EvalContext) -> EvalResult<Self> {
        use crate::diag::PushDiag;
        match (&parameter.specified_type, &parameter.default_value) {
            // Type and value are specified
            (Some(specified_type), Some(default_value)) => {
                let default_value = default_value.eval(context)?;
                if specified_type.ty() != default_value.ty() {
                    context.error(
                        parameter,
                        EvalError::ParameterTypeMismatch {
                            name: parameter.name.clone(),
                            expected: specified_type.ty(),
                            found: default_value.ty(),
                        },
                    )?;
                    // Return an invalid parameter value in case evaluation failed
                    Ok(ParameterValue::invalid(
                        parameter.name.clone(),
                        parameter.src_ref(),
                    ))
                } else {
                    Ok(ParameterValue::new(
                        parameter.name.clone(),
                        Some(specified_type.ty()),
                        Some(default_value),
                        parameter.src_ref(),
                    ))
                }
            }
            // Only type is specified
            (Some(t), None) => Ok(ParameterValue::new(
                parameter.name.clone(),
                Some(t.ty()),
                None,
                parameter.src_ref(),
            )),
            // Only value is specified
            (None, Some(expr)) => {
                let default_value = expr.eval(context)?;

                Ok(ParameterValue::new(
                    parameter.name.clone(),
                    Some(default_value.ty().clone()),
                    Some(default_value),
                    parameter.src_ref(),
                ))
            }
            // Neither type nor value is specified
            (None, None) => Ok(ParameterValue::invalid(
                parameter.name.clone(),
                parameter.src_ref(),
            )),
        }
    }
}

impl ParameterValueList {
    /// Create ParameterValueList from ParameterList
    pub fn from_parameter_list(
        parameters: &ParameterList,
        context: &mut EvalContext,
    ) -> EvalResult<Self> {
        let mut values = ParameterValueList::default();
        for parameter in parameters.iter() {
            values.push(ParameterValue::from_parameter(parameter, context)?)?;
        }

        Ok(values)
    }
}

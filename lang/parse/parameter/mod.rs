// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parameter parser entities

mod parameter_list;

use crate::{eval::*, ord_map::OrdMapValue, parse::*, parser::*, r#type::*, src_ref::*};

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
    src_ref: SrcRef,
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

impl Parse for Parameter {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut default_value = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::r#type => {
                    specified_type = Some(TypeAnnotation::parse(pair)?);
                }
                Rule::expression => {
                    default_value = Some(Expression::parse(pair)?);
                }
                rule => {
                    unreachable!(
                        "Unexpected token in parameter: {:?} {:?}",
                        rule,
                        pair.as_span().as_str()
                    );
                }
            }
        }

        if specified_type.is_none() && default_value.is_none() {
            return Err(ParseError::ParameterMissingTypeOrValue(name.clone()));
        }

        Ok(Self {
            name,
            specified_type,
            default_value,
            src_ref: pair.into(),
        })
    }
}

impl Eval for Parameter {
    type Output = ParameterValue;

    fn eval(&self, context: &mut EvalContext) -> EvalResult<Self::Output> {
        match (&self.specified_type, &self.default_value) {
            // Type and value are specified
            (Some(specified_type), Some(expr)) => {
                let default_value = expr.eval(context)?;
                if specified_type.ty() != default_value.ty() {
                    context.error_with_stack_trace(
                        self,
                        EvalError::ParameterTypeMismatch {
                            name: self.name.clone(),
                            expected: specified_type.ty(),
                            found: default_value.ty(),
                        },
                    )?;
                    // Return an invalid parameter value in case evaluation failed
                    Ok(ParameterValue::invalid(
                        self.name.id().clone(),
                        self.src_ref(),
                    ))
                } else {
                    Ok(ParameterValue::new(
                        self.name.id().clone(),
                        Some(specified_type.ty()),
                        Some(default_value),
                        self.src_ref(),
                    ))
                }
            }
            // Only type is specified
            (Some(t), None) => Ok(ParameterValue::new(
                self.name.id().clone(),
                Some(t.ty()),
                None,
                self.src_ref(),
            )),
            // Only value is specified
            (None, Some(expr)) => {
                let default_value = expr.eval(context)?;

                Ok(ParameterValue::new(
                    self.name.id().clone(),
                    Some(default_value.ty().clone()),
                    Some(default_value),
                    self.src_ref(),
                ))
            }
            // Neither type nor value is specified
            (None, None) => Ok(ParameterValue::new(
                self.name.id().clone(),
                None,
                None,
                self.src_ref(),
            )),
        }
    }
}

/// Short cut to create a `Parameter` instance
#[macro_export]
macro_rules! parameter {
    ($name:ident) => {
        microcad_lang::parse::Parameter::new(stringify!($name).into(), None, None, SrcRef(None))
    };
    ($name:ident: $ty:ident) => {
        microcad_lang::parse::Parameter::new(
            stringify!($name).into(),
            Some(microcad_lang::r#type::Type::$ty.into()),
            None,
            microcad_lang::src_ref::SrcRef(None),
        )
    };
    ($name:ident: $ty:ident = $value:expr) => {
        microcad_lang::parse::Parameter::new(
            stringify!($name).into(),
            Some(microcad_lang::r#type::Type::$ty.into()),
            Some(Expression::literal_from_str(stringify!($value)).expect("Invalid literal")),
            microcad_lang::src_ref::SrcRef(None),
        )
    };
}

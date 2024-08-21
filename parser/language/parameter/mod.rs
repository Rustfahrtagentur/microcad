mod parameter_list;
mod parameter_value;
mod parameter_value_list;

pub use parameter_list::*;
pub use parameter_value::*;
pub use parameter_value_list::*;

use crate::{eval::*, language::*, ord_map::OrdMapValue, parser::*, with_pair_ok};

/// @brief A parameter for a function or module definition
#[derive(Clone, Debug, Default)]
pub struct Parameter {
    pub name: Identifier,
    pub specified_type: Option<Type>,
    pub default_value: Option<Expression>,
}

impl OrdMapValue<Identifier> for Parameter {
    fn key(&self) -> Option<Identifier> {
        Some(self.name.clone())
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
            (Some(t), Some(v)) => write!(f, "{}: {t} = {v}", self.name)?,
            (Some(t), None) => write!(f, "{}: {t}", self.name)?,
            (None, Some(v)) => write!(f, "{} = {v}", self.name)?,
            _ => {}
        }

        write!(f, "{}", self.name)
    }
}

impl Parse for Parameter {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut default_value = None;

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?.value().clone();
                }
                Rule::r#type => {
                    specified_type = Some(Type::parse(pair)?.value().clone());
                }
                Rule::expression => {
                    default_value = Some(Expression::parse(pair)?.value().clone());
                }
                rule => {
                    unreachable!(
                        "Unexpected token in definition parameter: {:?} {:?}",
                        rule,
                        pair.as_span().as_str()
                    );
                }
            }
        }

        if specified_type.is_none() && default_value.is_none() {
            return Err(ParseError::ParameterMissingTypeOrValue(name.clone()));
        }

        with_pair_ok!(
            Self {
                name,
                specified_type,
                default_value,
            },
            pair
        )
    }
}

impl Eval for Parameter {
    type Output = ParameterValue;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        match (&self.specified_type, &self.default_value) {
            // Type and value are specified
            (Some(specified_type), Some(expr)) => {
                let default_value = expr.eval(context)?;
                if specified_type != &default_value.ty() {
                    Err(Error::ParameterTypeMismatch(
                        self.name.clone(),
                        specified_type.clone(),
                        default_value.ty(),
                    ))
                } else {
                    Ok(ParameterValue {
                        name: self.name.clone(),
                        specified_type: Some(specified_type.clone()),
                        default_value: Some(default_value),
                    })
                }
            }
            // Only type is specified
            (Some(t), None) => Ok(ParameterValue {
                name: self.name.clone(),
                specified_type: Some(t.clone()),
                default_value: None,
            }),
            // Only value is specified
            (None, Some(expr)) => {
                let default_value = expr.eval(context)?;

                Ok(ParameterValue {
                    name: self.name.clone(),
                    specified_type: Some(default_value.ty().clone()),
                    default_value: Some(default_value),
                })
            }
            // Neither type nor value is specified
            (None, None) => Ok(ParameterValue {
                name: self.name.clone(),
                specified_type: None,
                default_value: None,
            }),
        }
    }
}

#[macro_export]
macro_rules! parameter {
    ($name:ident) => {
        Parameter {
            name: stringify!($name).into(),
            specified_type: None,
            default_value: None,
        }
    };
    ($name:ident: $ty:ident) => {
        Parameter {
            name: stringify!($name).into(),
            specified_type: Some(Type::$ty),
            default_value: None,
        }
    };
    ($name:ident: $ty:ident = $value:expr) => {
        Parameter {
            name: stringify!($name).into(),
            specified_type: Some(Type::$ty),
            default_value: Some(
                Expression::literal_from_str(stringify!($value)).expect("Invalid literal"),
            ),
        }
    };
}

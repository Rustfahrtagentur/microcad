use std::ops::Deref;

use super::{expression::*, identifier::*, lang_type::*, value::*};
use crate::{eval::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
pub struct Parameter {
    name: Identifier,
    specified_type: Option<Type>,
    default_value: Option<Expression>,
}

impl Parameter {
    pub fn new(
        name: Identifier,
        specified_type: Option<Type>,
        default_value: Option<Expression>,
    ) -> Self {
        Self {
            name,
            specified_type,
            default_value,
        }
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn specified_type(&self) -> Option<&Type> {
        self.specified_type.as_ref()
    }

    pub fn default_value(&self) -> Option<&Expression> {
        self.default_value.as_ref()
    }
}

impl std::fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
            (Some(t), Some(v)) => write!(f, "{}: {} = {}", self.name, t, v)?,
            (Some(t), None) => write!(f, "{}: {}", self.name, t)?,
            (None, Some(v)) => write!(f, "{} = {}", self.name, v)?,
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
    type Output = (Option<Value>, Type);

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        match (&self.specified_type, &self.default_value) {
            (Some(specified_type), Some(expr)) => {
                let default_value = expr.eval(context)?;
                if specified_type != &default_value.ty() {
                    Err(Error::ParameterTypeMismatch(
                        self.name.clone(),
                        specified_type.clone(),
                        default_value.ty(),
                    ))
                } else {
                    Ok((Some(default_value), specified_type.clone()))
                }
            }
            (Some(t), None) => Ok((None, t.clone())),
            (None, Some(expr)) => {
                let default_value = expr.eval(context)?;
                Ok((Some(default_value.clone()), default_value.ty()))
            }
            (None, None) => Err(Error::ParameterMissingTypeOrValue(self.name.clone())),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ParameterList {
    parameters: Vec<Parameter>,
    by_name: std::collections::HashMap<String, usize>,
}

impl Parse for ParameterList {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::parameter_list);
        let mut parameters = Vec::new();
        let mut by_name = std::collections::HashMap::new();

        for pair in pair.clone().into_inner() {
            let parameter = Parameter::parse(pair)?.value().clone();
            if by_name.contains_key(&parameter.name().to_string()) {
                return Err(ParseError::DuplicateParameter(parameter.name().clone()));
            }

            by_name.insert(parameter.name().to_string(), parameters.len());
            parameters.push(parameter);
        }

        with_pair_ok!(
            Self {
                parameters,
                by_name
            },
            pair
        )
    }
}

impl Deref for ParameterList {
    type Target = Vec<Parameter>;

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

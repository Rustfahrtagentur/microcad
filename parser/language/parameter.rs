use std::ops::Deref;

use super::{expression::*, identifier::*, lang_type::*, value::*};
use crate::{eval::*, parser::*, with_pair_ok};

/// @brief A parameter for a function or module definition
#[derive(Clone, Debug)]
pub struct Parameter {
    name: Identifier,
    specified_type: Option<Type>,
    default_value: Option<Expression>,
}

impl Parameter {
    /// @brief Create a new parameter
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

/// @brief Parameter value is the result of evaluating a parameter
#[derive(Clone, Debug)]
pub struct ParameterValue {
    name: Identifier,
    specified_type: Option<Type>,
    default_value: Option<Value>,
}

pub enum TypeCheckResult {
    Ok,
    Tuple,
    List,
    Err(Error),
}

impl ParameterValue {
    pub fn new(
        name: Identifier,
        specified_type: Option<Type>,
        default_value: Option<Value>,
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

    pub fn default_value(&self) -> Option<&Value> {
        self.default_value.as_ref()
    }

    pub fn type_matches(&self, ty: &Type) -> bool {
        match &self.specified_type {
            Some(t) => t == ty,
            None => true, // Accept any type if none is specified
        }
    }

    pub fn type_check(&self, ty: &Type) -> TypeCheckResult {
        if self.type_matches(ty) {
            TypeCheckResult::Ok
        } else if ty.is_list_of(&self.specified_type.clone().unwrap()) {
            TypeCheckResult::List
        } else {
            TypeCheckResult::Err(Error::ParameterTypeMismatch(
                self.name.clone(),
                self.specified_type.clone().unwrap(),
                ty.clone(),
            ))
        }
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

#[derive(Clone, Debug, Default)]
pub struct ParameterList {
    parameters: Vec<Parameter>,
    by_name: std::collections::HashMap<String, usize>,
}

impl ParameterList {
    pub fn new(parameters: Vec<Parameter>) -> Self {
        let mut by_name = std::collections::HashMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            by_name.insert(parameter.name().to_string(), i);
        }

        Self {
            parameters,
            by_name,
        }
    }

    pub fn push(&mut self, parameter: Parameter) -> Result<(), ParseError> {
        if self.by_name.contains_key(&parameter.name().to_string()) {
            return Err(ParseError::DuplicateParameter(parameter.name().clone()));
        }

        self.by_name
            .insert(parameter.name().to_string(), self.parameters.len());
        self.parameters.push(parameter);
        Ok(())
    }
}

impl Parse for ParameterList {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::parameter_list);
        let mut parameters = ParameterList::default();

        for pair in pair.clone().into_inner() {
            parameters.push(Parameter::parse(pair)?.value().clone())?;
        }

        with_pair_ok!(parameters, pair)
    }
}

#[derive(Clone, Debug, Default)]
pub struct ParameterValueList {
    parameters: Vec<ParameterValue>,
    by_name: std::collections::HashMap<Identifier, usize>,
}

impl ParameterValueList {
    pub fn new(parameters: Vec<ParameterValue>) -> Self {
        let mut by_name = std::collections::HashMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            by_name.insert(parameter.name().clone(), i);
        }

        Self {
            parameters,
            by_name,
        }
    }

    pub fn push(&mut self, parameter: ParameterValue) -> Result<(), ParseError> {
        if self.by_name.contains_key(parameter.name()) {
            return Err(ParseError::DuplicateParameter(parameter.name().clone()));
        }

        self.by_name
            .insert(parameter.name().clone(), self.parameters.len());
        self.parameters.push(parameter);
        Ok(())
    }

    pub fn get(&self, name: &Identifier) -> Option<&ParameterValue> {
        self.by_name.get(name).map(|i| &self.parameters[*i])
    }

    pub fn remove(&mut self, name: &Identifier) {
        if let Some(new_index) = self.by_name.remove(name) {
            self.parameters.remove(new_index);
            for index in &mut self.by_name.values_mut() {
                if *index > new_index {
                    *index -= 1;
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.by_name.is_empty()
    }
}

impl Deref for ParameterValueList {
    type Target = Vec<ParameterValue>;

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

impl Eval for ParameterList {
    type Output = ParameterValueList;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        let mut values = ParameterValueList::default();
        for parameter in &self.parameters {
            values.push(parameter.eval(context)?).unwrap(); // Unwrap is safe here because we know the parameter is unique
        }

        Ok(values)
    }
}

impl Deref for ParameterList {
    type Target = Vec<Parameter>;

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

#[macro_export]
macro_rules! parameter {
    ($name:ident) => {
        Parameter::new(stringify!($name).into(), None, None)
    };
    ($name:ident: $ty:ident) => {
        Parameter::new(stringify!($name).into(), Some(Type::$ty), None)
    };
    ($name:ident: $ty:ident = $value:expr) => {
        Parameter::new(
            stringify!($name).into(),
            Some(Type::$ty),
            Some(Expression::literal_from_str(stringify!($value)).expect("Invalid literal")),
        )
    };
}

#[macro_export]
macro_rules! parameter_value {
    ($name:ident) => {
        ParameterValue::new(stringify!($name).into(), None, None)
    };
    ($name:ident: $ty:ident) => {
        $crate::language::call::ParameterValue::new(stringify!($name).into(), Some(Type::$ty), None)
    };
    ($name:ident: $ty:ident = $value:expr) => {
        $crate::language::call::ParameterValue::new(
            stringify!($name).into(),
            Some(Type::$ty),
            Some(Value::$ty($value)),
        )
    };
    ($name:ident = $value:expr) => {
        ParameterValue::new(stringify!($name).into(), None, Some($value))
    };
    () => {};
}

#[macro_export]
macro_rules! parameter_list {
    [$($param:expr),*] => {
        microcad_parser::language::parameter::ParameterList::new(vec![
            $($param,)*
        ])
    };
    ($($name:ident),*) => {
        microcad_parser::language::parameter_list![$(microcad_parser::parameter!($name)),*]
    };
    ($($name:ident: $ty:ident),*) => {
        microcad_parser::language::parameter_list![$(microcad_parser::parameter!($name: $ty)),*]
    };
    ($($name:ident: $ty:ident = $value:expr),*) => {
        microcad_parser::language::parameter_list![$(microcad_parser::parameter!($name: $ty = $value)),*]
    };
}

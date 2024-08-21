use crate::{eval::*, language::*, r#type::*};

/// @brief Parameter value is the result of evaluating a parameter
#[derive(Clone, Debug)]
pub struct ParameterValue {
    pub name: Identifier,
    pub specified_type: Option<Type>,
    pub default_value: Option<Value>,
}

pub enum TypeCheckResult {
    Ok,
    Tuple,
    List,
    Err(EvalError),
}

impl ParameterValue {
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
            TypeCheckResult::Err(EvalError::ParameterTypeMismatch(
                self.name.clone(),
                self.specified_type.clone().unwrap(),
                ty.clone(),
            ))
        }
    }
}

#[macro_export]
macro_rules! parameter_value {
    ($name:ident) => {
        ParameterValue {
            name: stringify!($name).into(),
            specified_type: None,
            default_value: None,
        }
    };
    ($name:ident: $ty:ident) => {
        $crate::eval::ParameterValue {
            name: stringify!($name).into(),
            specified_type: Some(Type::$ty),
            default_value: None,
        }
    };
    ($name:ident: $ty:ident = $value:expr) => {
        $crate::eval::ParameterValue {
            name: stringify!($name).into(),
            specified_type: Some(Type::$ty),
            default_value: Some(Value::$ty($value)),
        }
    };
    ($name:ident = $value:expr) => {
        ParameterValue::new(stringify!($name).into(), None, Some($value))
    };
    () => {};
}

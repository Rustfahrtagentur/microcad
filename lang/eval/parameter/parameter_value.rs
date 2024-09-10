// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Parameter value evaluation entity
use crate::{eval::*, r#type::*, src_ref::*};

/// Parameter value is the result of evaluating a parameter
#[derive(Clone, Debug)]
pub struct ParameterValue {
    /// Parameter name
    pub name: Id,
    /// Parameter type
    pub specified_type: Option<Type>,
    /// Parameter default
    pub default_value: Option<Value>,
    /// Source code reference
    src_ref: SrcRef,
}

pub enum TypeCheckResult {
    Ok,
    Tuple,
    List,
    Err(EvalError),
}

impl ParameterValue {
    pub fn new(
        name: Id,
        specified_type: Option<Type>,
        default_value: Option<Value>,
        src_ref: SrcRef,
    ) -> Self {
        Self {
            name,
            specified_type,
            default_value,
            src_ref,
        }
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
            TypeCheckResult::Err(EvalError::ParameterTypeMismatch(
                self.name.clone(),
                self.specified_type.clone().unwrap(),
                ty.clone(),
            ))
        }
    }
}

impl SrcReferrer for ParameterValue {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! parameter_value {
    ($name:ident) => {
        ParameterValue {
            name: stringify!($name).into(),
            specified_type: None,
            default_value: None,
            SrcRef(None),
        }
    };
    ($name:ident: $ty:ident) => {
        ParameterValue::new(
            stringify!($name).into(),
            Some(Type::$ty),
            None,
            SrcRef(None),
        )
    };
    ($name:ident: $ty:ident = $value:expr) => {
        ParameterValue::new(
            stringify!($name).into(),
            Some(Type::$ty),
            Some(Value::$ty(Refer::none($value))),
            SrcRef(None),
        )
    };
    ($name:ident = $value:expr) => {
        ParameterValue::new(stringify!($name).into(), None, Some($value), SrcRef(None))
    };
    () => {};
}


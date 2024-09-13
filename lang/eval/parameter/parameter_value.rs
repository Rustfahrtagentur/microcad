// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value evaluation entity

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

/// Result of a type check with `ParameterValue::type_check()`
pub enum TypeCheckResult {
    /// Self's type matched given type
    Match,
    /// Self is list of that type
    List,
    /// An error occurred
    NoMatch(Id, Type, Type),
}

impl ParameterValue {
    /// Create new parameter value
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

    /// Check how the type of this parameter value relates to the given one
    /// # Return
    /// - `TypeCheckResult::Match`: Given type matches exactly
    /// - `TypeCheckResult::List`: Given type is a list of items of a type that matches exactly
    /// - `TypeCheckResult::NoMatch(err)`: Types do not match (`err` describes both type
    pub fn type_check(&self, ty: &Type) -> TypeCheckResult {
        if self.type_matches(ty) {
            TypeCheckResult::Match
        } else if ty.is_list_of(&self.specified_type.clone().unwrap()) {
            TypeCheckResult::List
        } else {
            TypeCheckResult::NoMatch(
                self.name.clone(),
                self.specified_type.clone().unwrap(),
                ty.clone(),
            )
        }
    }

    /// Check if type of this parameter value matches the given one
    fn type_matches(&self, ty: &Type) -> bool {
        match &self.specified_type {
            Some(t) => t == ty,
            None => true, // Accept any type if none is specified
        }
    }
}

impl SrcReferrer for ParameterValue {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

/// Shortcut to create a `ParameterValue`
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

// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value evaluation entity

use crate::{src_ref::*, ty::*, value::*};

/// Parameter value is the result of evaluating a parameter
#[derive(Clone, Debug)]
pub struct ParameterValue {
    /// Parameter name
    pub id: Identifier,
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
    SingleMatch,
    /// Self is list of that type
    MultiMatch,
    /// An error occurred
    NoMatch(Identifier, Option<Type>, Type),
}

impl ParameterValue {
    /// Create new parameter value
    pub fn new(
        id: Identifier,
        specified_type: Option<Type>,
        default_value: Option<Value>,
        src_ref: SrcRef,
    ) -> Self {
        Self {
            id,
            specified_type,
            default_value,
            src_ref,
        }
    }

    /// Creates an invalid parameter value, in case an error occured during evaluation
    pub fn invalid(id: Identifier, src_ref: SrcRef) -> Self {
        Self {
            id,
            specified_type: None,
            default_value: None,
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
            return TypeCheckResult::SingleMatch;
        }

        if let Some(specified_type) = self.specified_type.as_ref() {
            if ty.is_list_of(specified_type) {
                TypeCheckResult::MultiMatch
            } else {
                TypeCheckResult::NoMatch(self.id.clone(), Some(specified_type.clone()), ty.clone())
            }
        } else {
            TypeCheckResult::NoMatch(self.id.clone(), None, ty.clone())
        }
    }

    /// Check if type of this parameter value matches the given one
    pub fn type_matches(&self, ty: &Type) -> bool {
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
    ($id:ident) => {
        $crate::value::ParameterValue {
            name: stringify!($id).into(),
            specified_type: None,
            default_value: None,
            SrcRef(None),
        }
    };
    ($id:ident: $ty:ident) => {
        $crate::value::ParameterValue::new(
            stringify!($id).into(),
            Some(Type::$ty),
            None,
            SrcRef(None),
        )
    };
    ($id:ident: $ty:ident = $value:expr) => {
        $crate::value::ParameterValue::new(
            stringify!($id).into(),
            Some(Type::$ty),
            Some($crate::value::Value::$ty(Refer::none($value))),
            SrcRef(None),
        )
    };
    ($id:ident = $value:expr) => {
        value::ParameterValue::new(stringify!($id).into(), None, Some($value), SrcRef(None))
    };
    () => {};
}

#[test]
fn test_is_list_of() {
    use crate::syntax::*;

    crate::env_logger_init();

    assert!(Type::List(ListType::new(Type::Scalar)).is_list_of(&Type::Scalar));
}

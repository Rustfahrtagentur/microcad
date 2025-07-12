// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value evaluation entity

use crate::{src_ref::*, ty::*, value::*};

/// Parameter value is the result of evaluating a parameter
#[derive(Clone, Debug, Default)]
pub struct ParameterValue {
    /// Parameter type
    pub specified_type: Option<Type>,
    /// Parameter default
    pub default_value: Option<Value>,
    /// Source code reference
    pub src_ref: SrcRef,
}

/// Result of a type check with `ParameterValue::type_check()`
pub enum TypeCheckResult {
    /// Self's type matched given type
    SingleMatch,
    /// Self is list of that type
    MultiMatch,
    /// An error occurred
    NoMatch(Option<Type>, Type),
}

impl ParameterValue {
    /// Creates an invalid parameter value, in case an error occurred during evaluation
    pub fn invalid(src_ref: SrcRef) -> Self {
        Self {
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
                TypeCheckResult::NoMatch(Some(specified_type.clone()), ty.clone())
            }
        } else {
            TypeCheckResult::NoMatch(None, ty.clone())
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

impl Ty for ParameterValue {
    /// Return effective type
    ///
    /// Returns any `specified_type` or the type of the `default_value`.
    /// Panics if neither of both is available.
    fn ty(&self) -> Type {
        if let Some(ty) = &self.specified_type {
            ty.clone()
        } else if let Some(def) = &self.default_value {
            def.ty()
        } else {
            log::error!("type of parameter value cannot be achieved");
            Type::Invalid
        }
    }
}

impl std::fmt::Display for ParameterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(def) = &self.default_value {
            write!(f, "{} = {}", def.ty(), def.value_to_string())?;
        } else if let Some(ty) = &self.specified_type {
            write!(f, "{ty}")?;
        }
        Ok(())
    }
}

impl SrcReferrer for ParameterValue {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

#[test]
fn test_is_list_of() {
    assert!(Type::List(ListType::new(QuantityType::Scalar.into()))
        .is_list_of(&QuantityType::Scalar.into()));
}

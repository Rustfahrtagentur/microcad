// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin module

pub mod attributes;
pub mod color;
pub mod module_builder;

use microcad_core::*;
pub use module_builder::*;

use crate::{ty::*, value::*};

/// This enum is used to declare parameter list for builtin symbols conveniently.
///
/// It is used in the [`parameter_value`] and [`argument_value`] macros to be able
/// to declare parameters and arguments in µcad like way, for example: `a: Scalar = 4.0`.
pub enum BuiltinTypeHelper {
    /// Integer type.
    Integer,
    /// Scalar type.
    Scalar,
    /// Angle type.
    Angle,
    /// String type.
    String,
}

impl From<BuiltinTypeHelper> for Type {
    fn from(value: BuiltinTypeHelper) -> Self {
        match value {
            BuiltinTypeHelper::Integer => Type::Integer,
            BuiltinTypeHelper::Scalar => Type::Quantity(QuantityType::Scalar),
            BuiltinTypeHelper::Angle => Type::Quantity(QuantityType::Angle),
            BuiltinTypeHelper::String => Type::String,
        }
    }
}

/// This enum is used to declare parameter list for builtin symbols conveniently.
///
/// It is used in the [`parameter_value`] and [`argument_value`] macros to be able
/// to declare parameters and arguments in µcad like way, for example: `a: Scalar = 4.0`.
pub enum BuiltinValueHelper {
    /// Integer type.
    Integer(Integer),
    /// Scalar type.
    Scalar(Scalar),
    /// Length type.
    Length(Scalar),
    /// String type.
    String(String),
}

impl From<BuiltinValueHelper> for Value {
    fn from(value: BuiltinValueHelper) -> Self {
        match value {
            BuiltinValueHelper::Scalar(v) => {
                Value::Quantity(Quantity::new(v, QuantityType::Scalar))
            }
            BuiltinValueHelper::Integer(i) => Value::Integer(i),
            BuiltinValueHelper::Length(v) => {
                Value::Quantity(Quantity::new(v, QuantityType::Length))
            }
            BuiltinValueHelper::String(s) => Value::String(s),
        }
    }
}

/// Shortcut to create a `ParameterValue`
#[macro_export]
macro_rules! parameter_value {
    ($id:ident) => {
        $crate::eval::ParameterValue::new(
            $crate::syntax::Identifier::no_ref(stringify!($id)),
            None,
            None,
            $crate::src_ref::SrcRef(None),
        )
    };
    ($id:ident: $ty:ident) => {
        $crate::eval::ParameterValue::new(
            $crate::syntax::Identifier::no_ref(stringify!($id)),
            Some($crate::builtin::BuiltinTypeHelper::$ty.into()),
            None,
            $crate::src_ref::SrcRef(None),
        )
    };
    ($id:ident: $ty:ident = $value:expr) => {
        $crate::eval::ParameterValue::new(
            $crate::syntax::Identifier::no_ref(stringify!($id)),
            Some($crate::builtin::BuiltinTypeHelper::$ty.into()),
            Some($crate::builtin::BuiltinValueHelper::$ty($value).into()),
            $crate::src_ref::SrcRef(None),
        )
    };
    ($id:ident = $value:expr) => {
        value::ParameterValue::new(
            $crate::syntax::Identifier::no_ref(stringify!($id)),
            None,
            Some($value),
            SrcRef(None),
        )
    };
    () => {};
}

/// Shortcut to create a argument value
#[macro_export]
macro_rules! argument_value {
    ($name:ident: $ty:ident = $value:expr) => {
        ArgumentValue::new(
            Some(stringify!($name).into()),
            $crate::builtin::BuiltinValueHelper::$ty($value).into(),
            $crate::src_ref::SrcRef(None),
        )
    };
    ($ty:ident = $value:expr) => {
        ArgumentValue::new(
            None,
            $crate::builtin::BuiltinValueHelper::$ty($value).into(),
            $crate::src_ref::SrcRef(None),
        )
    };
    () => {};
}

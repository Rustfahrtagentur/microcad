// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin module

pub mod export;
pub mod file_io;
pub mod import;
pub mod module_builder;

pub use export::*;
pub use file_io::*;
pub use import::*;
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
    /// A unitless scalar value.
    Scalar,
    /// Length in mm.
    Length,
    /// Area in mm².
    Area,
    /// Volume in mm³.
    Volume,
    /// Density in g/mm³
    Density,
    /// An angle in radians.
    Angle,
    /// Weight of a specific volume of material.
    Weight,
    /// String type.
    String,
    /// Bool type
    Bool,
    /// Color type
    Color,
}

impl From<BuiltinTypeHelper> for Type {
    fn from(value: BuiltinTypeHelper) -> Self {
        match value {
            BuiltinTypeHelper::Integer => Type::Integer,
            BuiltinTypeHelper::Scalar => Type::Quantity(QuantityType::Scalar),
            BuiltinTypeHelper::Length => Type::Quantity(QuantityType::Length),
            BuiltinTypeHelper::Area => Type::Quantity(QuantityType::Area),
            BuiltinTypeHelper::Volume => Type::Quantity(QuantityType::Volume),
            BuiltinTypeHelper::Density => Type::Quantity(QuantityType::Density),
            BuiltinTypeHelper::Angle => Type::Quantity(QuantityType::Angle),
            BuiltinTypeHelper::Weight => Type::Quantity(QuantityType::Weight),
            BuiltinTypeHelper::String => Type::String,
            BuiltinTypeHelper::Bool => Type::Bool,
            BuiltinTypeHelper::Color => Type::Tuple(TupleType::new_color().into()),
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
    /// Area type
    Area(Scalar),
    /// Volume type
    Volume(Scalar),
    /// Density type
    Density(Scalar),
    /// Angle type
    Angle(Scalar),
    /// Weight type
    Weight(Scalar),
    /// String type.
    String(String),
    /// Bool type
    Bool(bool),
    /// Color type
    Color(Color),
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
            BuiltinValueHelper::Area(v) => Value::Quantity(Quantity::new(v, QuantityType::Area)),
            BuiltinValueHelper::Volume(v) => {
                Value::Quantity(Quantity::new(v, QuantityType::Volume))
            }
            BuiltinValueHelper::Density(v) => {
                Value::Quantity(Quantity::new(v, QuantityType::Density))
            }
            BuiltinValueHelper::Angle(v) => Value::Quantity(Quantity::new(v, QuantityType::Angle)),
            BuiltinValueHelper::Weight(v) => {
                Value::Quantity(Quantity::new(v, QuantityType::Weight))
            }
            BuiltinValueHelper::String(s) => Value::String(s),
            BuiltinValueHelper::Bool(b) => Value::Bool(b),
            BuiltinValueHelper::Color(c) => crate::tuple_value!(r = c.r, g = c.g, b = c.b, a = c.a),
        }
    }
}

/// Shortcut to create a `ParameterValue`
#[macro_export]
macro_rules! parameter {
    ($id:ident) => {
        (
            $crate::syntax::Identifier::no_ref(stringify!($id)),
            $crate::eval::ParameterValue {
                src_ref: $crate::src_ref::SrcRef(None),
                ..Default::default()
            },
        )
    };
    ($id:ident: $ty:ident) => {
        (
            $crate::syntax::Identifier::no_ref(stringify!($id)),
            $crate::eval::ParameterValue {
                specified_type: Some($crate::builtin::BuiltinTypeHelper::$ty.into()),
                src_ref: $crate::src_ref::SrcRef(None),
                ..Default::default()
            },
        )
    };
    ($id:ident: $ty:ident = $value:expr) => {
        (
            $crate::syntax::Identifier::no_ref(stringify!($id)),
            $crate::eval::ParameterValue {
                specified_type: Some($crate::builtin::BuiltinTypeHelper::$ty.into()),
                default_value: Some($crate::builtin::BuiltinValueHelper::$ty($value).into()),
                src_ref: $crate::src_ref::SrcRef(None),
            },
        )
    };
    ($id:ident = $value:expr) => {
        (
            $crate::syntax::Identifier::no_ref(stringify!($id)),
            $crate::eval::ParameterValue {
                default_value: Some($value),
                ..Default::default()
            },
        )
    };
    () => {};
}

/// Shortcut to create a argument value
#[macro_export]
macro_rules! argument {
    ($id:ident: $ty:ident = $value:expr) => {
        (
            $crate::syntax::Identifier::no_ref(stringify!($id)),
            ArgumentValue::new(
                $crate::builtin::BuiltinValueHelper::$ty($value).into(),
                $crate::src_ref::SrcRef(None),
            ),
        )
    };
    ($ty:ident = $value:expr) => {
        (
            Identifier::none(),
            ArgumentValue::new(
                $crate::builtin::BuiltinValueHelper::$ty($value).into(),
                $crate::src_ref::SrcRef(None),
            ),
        )
    };
    () => {};
}

/// Create tuple of stringified `Identifier` and a `Value`
#[macro_export]
macro_rules! property {
    ($id:ident : $ty:ident = $value:expr) => {
        (
            Identifier::no_ref(stringify!($id)),
            $crate::builtin::BuiltinValueHelper::$ty($value).into(),
        )
    };
    () => {};
}

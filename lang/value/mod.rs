// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation entities.
//!
//! Every evaluation of any *symbol* leads to a [`Value`] which then might continued
//! to process or ends up as the overall evaluation result.

mod into_value;
mod list;
mod map;
mod map_key_value;
mod named_tuple;

#[macro_use]
mod parameter_value;
mod parameter_value_list;
mod unnamed_tuple;
mod value_error;
mod value_list;

pub use into_value::IntoValue;
pub use list::*;
pub use map::*;
pub use map_key_value::*;
pub use named_tuple::*;
pub use parameter_value::*;
pub use parameter_value_list::*;
pub use unnamed_tuple::*;
pub use value_error::*;
pub use value_list::*;

use crate::{objects::*, src_ref::*, syntax::*, ty::*};
use cgmath::InnerSpace;
use microcad_core::*;

pub(crate) type ValueResult<Type = Value> = std::result::Result<Type, ValueError>;

/// A variant value with attached source code reference.
#[derive(Clone, Default, PartialEq)]
pub enum Value {
    /// Invalid value (used for error handling).
    #[default]
    None,
    /// An integer value.
    Integer(Integer),
    /// A scalar value.
    Scalar(Scalar),
    /// Length in mm.
    Length(Scalar),
    /// Area in mm².
    Area(Scalar),
    /// Volume in mm³.
    Volume(Scalar),
    /// A 2D vector with length.
    Vec2(Vec2),
    /// A 3D vector with length.
    Vec3(Vec3),
    /// A 4D vector with length.
    Vec4(Vec4),
    /// An angle in radians.
    Angle(Scalar),
    /// Weight of a specific volume of material.
    Weight(Scalar),
    /// A boolean value.
    Bool(bool),
    /// A string value.
    String(String),
    /// A color value.
    Color(Color),
    /// A list of values.
    List(List),
    /// A map of values.
    Map(Map),
    /// A tuple of named items.
    NamedTuple(NamedTuple),
    /// A tuple of unnamed items.
    UnnamedTuple(UnnamedTuple),
    /// A node in the render tree.
    Node(ObjectNode),
    /// A node list in the render tree, result from multiplicity.
    NodeMultiplicity(Vec<ObjectNode>),
}

impl Value {
    /// Add a unit to a primitive value (Scalar or Integer).
    pub fn bundle_unit(self, unit: Unit) -> ValueResult {
        match (self, unit.ty()) {
            (Value::Integer(i), Type::Length) => Ok(Value::Length(unit.normalize(i as Scalar))),
            (Value::Integer(i), Type::Angle) => Ok(Value::Angle(unit.normalize(i as Scalar))),
            (Value::Scalar(s), Type::Length) => Ok(Value::Length(unit.normalize(s))),
            (Value::Scalar(s), Type::Angle) => Ok(Value::Angle(unit.normalize(s))),
            (value, Type::Scalar) | (value, Type::Integer) => Ok(value),
            (value, _) => Err(ValueError::CannotAddUnitToValueWithUnit(value.clone())),
        }
    }

    /// Fetch nodes from this value.
    pub fn fetch_nodes(self) -> Vec<ObjectNode> {
        match self {
            Self::Node(n) => vec![n],
            Self::NodeMultiplicity(n) => n,
            _ => vec![],
        }
    }

    /// Get property value for a value.
    ///
    /// - `identifier`: Identifier of the property
    ///
    /// This function is used when accessing a property `p` of a value `v` with `p.v`.
    pub fn get_property_value(&self, identifier: &Identifier) -> Option<Value> {
        let get = |value| Some(Value::Scalar(value));
        let id = identifier.id().as_str();
        match self {
            Value::Vec2(r) => match id {
                "x" => get(r.x),
                "y" => get(r.y),
                _ => None,
            },
            Value::Vec3(r) => match id {
                "x" => get(r.x),
                "y" => get(r.y),
                "z" => get(r.z),
                _ => None,
            },
            Value::Vec4(r) => match id {
                "x" => get(r.x),
                "y" => get(r.y),
                "z" => get(r.z),
                "w" => get(r.w),
                _ => None,
            },
            Value::Color(r) => match id {
                "r" => get(r.r as f64),
                "g" => get(r.g as f64),
                "b" => get(r.b as f64),
                "a" => get(r.a as f64),
                _ => None,
            },
            Value::NamedTuple(named_tuple) => named_tuple.get(identifier).cloned(),
            Value::Node(node) => node.borrow().get_property_value(identifier).cloned(),
            _ => None,
        }
    }

    /// Check if the value is invalid.
    pub fn is_invalid(&self) -> bool {
        matches!(self, Value::None)
    }

    /// Binary operation
    pub fn binary_op(lhs: Value, rhs: Value, op: &str) -> ValueResult {
        match match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            "^" => unimplemented!(), // lhs.pow(&rhs),
            "&" => lhs & rhs,
            "|" => lhs | rhs,
            ">" => Ok(Value::Bool(lhs > rhs)),
            "<" => Ok(Value::Bool(lhs < rhs)),
            "≤" => Ok(Value::Bool(lhs <= rhs)),
            "≥" => Ok(Value::Bool(lhs >= rhs)),
            "~" => todo!("implement near ~="),
            "=" => Ok(Value::Bool(lhs == rhs)),
            "!=" => Ok(Value::Bool(lhs != rhs)),
            _ => unimplemented!("{op:?}"),
        } {
            Err(err) => Err(err),
            result => result,
        }
    }

    /// Unary operation.
    pub fn unary_op(self, op: &str) -> ValueResult {
        match op {
            "-" => -self,
            _ => unimplemented!(),
        }
    }

    /// Try to get boolean value.
    ///
    /// A `Value::None` will return false.
    pub fn try_bool(&self) -> Result<bool, ValueError> {
        match self {
            Value::Bool(b) => Ok(*b),
            Value::None => Ok(false),
            value => Err(ValueError::CannotConvertToBool(value.clone())),
        }
    }

    /// Try to convert into [Color].
    pub fn try_color(&self) -> Result<Color, ValueError> {
        match self {
            Value::String(s) => {
                if let Ok(color) = std::str::FromStr::from_str(s) {
                    return Ok(color);
                }
            }
            Value::Color(color) => return Ok(*color),
            _ => {}
        }

        Err(ValueError::CannotConvert(self.clone(), "Color".into()))
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // integer type
            (Value::Integer(lhs), Value::Integer(rhs)) => lhs.partial_cmp(rhs),

            // floating point types
            (Value::Scalar(lhs), Value::Scalar(rhs))
            | (Value::Length(lhs), Value::Length(rhs))
            | (Value::Area(lhs), Value::Area(rhs))
            | (Value::Volume(lhs), Value::Volume(rhs))
            | (Value::Angle(lhs), Value::Angle(rhs))
            | (Value::Angle(lhs), Value::Scalar(rhs))
            | (Value::Scalar(lhs), Value::Angle(rhs)) => lhs.partial_cmp(rhs),

            // vector types
            (Value::Vec2(lhs), Value::Vec2(rhs)) => lhs.magnitude2().partial_cmp(&rhs.magnitude2()),
            (Value::Vec3(lhs), Value::Vec3(rhs)) => lhs.magnitude2().partial_cmp(&rhs.magnitude2()),

            _ => {
                log::warn!("unhandled type mismatch between {self} and {other}");
                None
            }
        }
    }
}

impl crate::ty::Ty for Value {
    fn ty(&self) -> Type {
        match self {
            Value::None => Type::Invalid,
            Value::Integer(_) => Type::Integer,
            Value::Scalar(_) => Type::Scalar,
            Value::Length(_) => Type::Length,
            Value::Area(_) => Type::Area,
            Value::Volume(_) => Type::Volume,
            Value::Vec2(_) => Type::Vec2,
            Value::Vec3(_) => Type::Vec3,
            Value::Vec4(_) => Type::Vec4,
            Value::Angle(_) => Type::Angle,
            Value::Weight(_) => Type::Weight,
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Color(_) => Type::Color,
            Value::List(list) => list.ty(),
            Value::Map(map) => map.ty(),
            Value::NamedTuple(named_tuple) => named_tuple.ty(),
            Value::UnnamedTuple(unnamed_tuple) => unnamed_tuple.ty(),
            Value::Node(_) => Type::Node,
            Value::NodeMultiplicity(_) => Type::NodeMultiplicity,
        }
    }
}

impl std::ops::Neg for Value {
    type Output = ValueResult;

    fn neg(self) -> Self::Output {
        match self {
            Value::Integer(n) => Ok(Value::Integer(-n)),
            Value::Scalar(n) => Ok(Value::Scalar(-n)),
            Value::Length(n) => Ok(Value::Length(-n)),
            Value::Vec2(v) => Ok(Value::Vec2(-v)),
            Value::Vec3(v) => Ok(Value::Vec3(-v)),
            Value::Angle(n) => Ok(Value::Angle(-n)),
            _ => Err(ValueError::InvalidOperator("-".into())),
        }
    }
}

/// Rules for operator `+`.
impl std::ops::Add for Value {
    type Output = ValueResult;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Add two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs + rhs)),
            // Add a scalar to an integer
            (Value::Integer(lhs), Value::Scalar(rhs)) => Ok(Value::Scalar(lhs as Scalar + rhs)),
            // Add an integer to a scalar
            (Value::Scalar(lhs), Value::Integer(rhs)) => Ok(Value::Scalar(lhs + rhs as Scalar)),
            // Add two scalars
            (Value::Scalar(lhs), Value::Scalar(rhs)) => Ok(Value::Scalar(lhs + rhs)),
            // Add two angles
            (Value::Angle(lhs), Value::Angle(rhs)) => Ok(Value::Angle(lhs + rhs)),
            // Add two lengths
            (Value::Length(lhs), Value::Length(rhs)) => Ok(Value::Length(lhs + rhs)),
            // Add two Vec2
            (Value::Vec2(lhs), Value::Vec2(rhs)) => Ok(Value::Vec2(lhs + rhs)),
            // Add two Vec3
            (Value::Vec3(lhs), Value::Vec3(rhs)) => Ok(Value::Vec3(lhs + rhs)),
            // Concatenate two strings
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::String(lhs + &rhs)),
            // Concatenate two lists
            (Value::List(lhs), Value::List(rhs)) => {
                if lhs.ty() != rhs.ty() {
                    return Err(ValueError::CannotCombineVecOfDifferentType(
                        lhs.ty(),
                        rhs.ty(),
                    ));
                }

                Ok(Value::List(List::new(
                    lhs.iter().chain(rhs.iter()).cloned().collect(),
                    lhs.ty(),
                )))
            }
            // Add values of two tuples of the same length
            (Value::UnnamedTuple(lhs), Value::UnnamedTuple(rhs)) => {
                Ok(Value::UnnamedTuple((lhs + rhs)?))
            }
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} + {rhs}"))),
        }
    }
}

/// Rules for operator `-`.
impl std::ops::Sub for Value {
    type Output = ValueResult;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Subtract two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs - rhs)),
            // Subtract an scalar and an integer
            (Value::Scalar(lhs), Value::Integer(rhs)) => Ok(Value::Scalar(lhs - rhs as Scalar)),
            // Subtract an integer and a scalar
            (Value::Integer(lhs), Value::Scalar(rhs)) => Ok(Value::Scalar(lhs as Scalar - rhs)),
            // Subtract two numbers
            (Value::Scalar(lhs), Value::Scalar(rhs)) => Ok(Value::Scalar(lhs - rhs)),
            // Subtract two angles
            (Value::Angle(lhs), Value::Angle(rhs)) => Ok(Value::Angle(lhs - rhs)),
            // Subtract two lengths
            (Value::Length(lhs), Value::Length(rhs)) => Ok(Value::Length(lhs - rhs)),
            // Subtract two Vec2
            (Value::Vec2(lhs), Value::Vec2(rhs)) => Ok(Value::Vec2(lhs - rhs)),
            // Subtract two Vec3
            (Value::Vec3(lhs), Value::Vec3(rhs)) => Ok(Value::Vec3(lhs - rhs)),
            // Remove an elements from list `rhs` from list `lhs`
            (Value::List(mut lhs), Value::List(rhs)) => {
                if lhs.ty() == rhs.ty() {
                    lhs.retain(|x| !rhs.contains(x));
                    Ok(Value::List(lhs))
                } else {
                    Err(ValueError::CannotCombineVecOfDifferentType(
                        lhs.ty(),
                        rhs.ty(),
                    ))
                }
            }
            // Subtract values of two arrays of the same length
            (Value::UnnamedTuple(lhs), Value::UnnamedTuple(rhs)) => {
                Ok(Value::UnnamedTuple((lhs - rhs)?))
            }
            (Value::Node(lhs), Value::Node(rhs)) => {
                Ok(Value::Node(crate::objects::algorithm::binary_op(
                    microcad_core::BooleanOp::Difference,
                    lhs,
                    rhs,
                )))
            }
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} - {rhs}"))),
        }
    }
}

/// Rules for operator `*`.
impl std::ops::Mul for Value {
    type Output = ValueResult;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Multiply two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs * rhs)),
            // Multiply an integer and a scalar, result is scalar
            (Value::Integer(lhs), Value::Scalar(rhs)) => Ok(Value::Scalar(lhs as Scalar * rhs)),
            // Multiply a scalar and an integer, result is scalar
            (Value::Scalar(lhs), Value::Integer(rhs)) => Ok(Value::Scalar(lhs * rhs as Scalar)),
            // Multiply two scalars
            (Value::Scalar(lhs), Value::Scalar(rhs)) => Ok(Value::Scalar(lhs * rhs)),
            // Scale an angle with a scalar
            (Value::Angle(lhs), Value::Scalar(rhs)) | (Value::Scalar(rhs), Value::Angle(lhs)) => {
                Ok(Value::Angle(lhs * rhs))
            }
            // Scale an angle with an integer
            (Value::Angle(lhs), Value::Integer(rhs)) => Ok(Value::Scalar(lhs * rhs as Scalar)),
            // Scale an integer with an angle
            (Value::Integer(lhs), Value::Angle(rhs)) => Ok(Value::Scalar(lhs as Scalar * rhs)),
            // Scale a length with a scalar
            (Value::Length(lhs), Value::Scalar(rhs)) | (Value::Scalar(rhs), Value::Length(lhs)) => {
                Ok(Value::Length(lhs * rhs))
            }
            // Scale a length with an integer
            (Value::Length(lhs), Value::Integer(rhs)) => Ok(Value::Length(lhs * rhs as Scalar)),
            // Scale an integer with a length
            (Value::Integer(lhs), Value::Length(rhs)) => Ok(Value::Length(lhs as Scalar * rhs)),
            // Scale Vec2
            (Value::Scalar(lhs), Value::Vec2(rhs)) | (Value::Vec2(rhs), Value::Scalar(lhs)) => {
                Ok(Value::Vec2(Vec2::new(lhs * rhs.x, lhs * rhs.y)))
            }
            // Scale Vec3
            (Value::Scalar(lhs), Value::Vec3(rhs)) | (Value::Vec3(rhs), Value::Scalar(lhs)) => Ok(
                Value::Vec3(Vec3::new(lhs * rhs.x, lhs * rhs.y, lhs * rhs.z)),
            ),
            (Value::List(list), value) | (value, Value::List(list)) => Ok(list * value)?,
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} * {rhs}"))),
        }
    }
}

/// Rules for operator `/`.
impl std::ops::Div for Value {
    type Output = ValueResult;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Division with scalar result
            (Value::Integer(lhs), Value::Integer(rhs)) => {
                Ok(Value::Scalar(lhs as Scalar / rhs as Scalar))
            }
            (Value::Scalar(lhs), Value::Integer(rhs)) => Ok(Value::Scalar(lhs / rhs as Scalar)),
            (Value::Integer(lhs), Value::Scalar(rhs)) => Ok(Value::Scalar(lhs as Scalar / rhs)),
            (Value::Scalar(lhs), Value::Scalar(rhs))
            | (Value::Length(lhs), Value::Length(rhs))
            | (Value::Angle(lhs), Value::Angle(rhs)) => Ok(Value::Scalar(lhs / rhs)),
            (Value::Length(lhs), Value::Scalar(rhs)) => Ok(Value::Length(lhs / rhs)),
            (Value::Angle(lhs), Value::Scalar(rhs)) => Ok(Value::Angle(lhs / rhs)),
            (Value::Length(lhs), Value::Integer(rhs)) => Ok(Value::Length(lhs / rhs as Scalar)),
            (Value::Angle(lhs), Value::Integer(rhs)) => Ok(Value::Angle(lhs / rhs as Scalar)),
            (Value::List(list), value) => Ok(list / value)?,
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} / {rhs}"))),
        }
    }
}

/// Rules for operator `|`` (union).
impl std::ops::BitOr for Value {
    type Output = ValueResult;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Node(lhs), Value::Node(rhs)) => Ok(Value::Node(
                crate::objects::algorithm::binary_op(BooleanOp::Union, lhs, rhs),
            )),
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} | {rhs}"))),
        }
    }
}

/// Rules for operator `&` (intersection).
impl std::ops::BitAnd for Value {
    type Output = ValueResult;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Node(lhs), Value::Node(rhs)) => Ok(Value::Node(
                crate::objects::algorithm::binary_op(BooleanOp::Intersection, lhs, rhs),
            )),
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} & {rhs}"))),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::None => write!(f, "<invalid>"),
            Value::Integer(n) => write!(f, "{n}"),
            Value::Scalar(n) => write!(f, "{n}"),
            Value::Length(n)
            | Value::Angle(n)
            | Value::Area(n)
            | Value::Volume(n)
            | Value::Weight(n) => {
                write!(f, "{n}{}", self.ty().default_unit())
            }
            Value::Vec2(v) => write!(f, "({}, {})", v.x, v.y),
            Value::Vec3(v) => write!(f, "({}, {}, {})", v.x, v.y, v.z),
            Value::Vec4(v) => write!(f, "({}, {}, {}, {})", v.x, v.y, v.z, v.w),
            Value::Bool(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Color(c) => write!(f, "{c}"),
            Value::List(l) => write!(f, "{l}"),
            Value::Map(m) => write!(f, "{m}"),
            Value::NamedTuple(t) => write!(f, "{t}"),
            Value::UnnamedTuple(t) => write!(f, "{t}"),
            Value::Node(n) => dump(f, n.clone()),
            Value::NodeMultiplicity(nm) => {
                writeln!(f, "Multiplicity [")?;
                for n in nm {
                    dump(f, n.clone())?;
                    write!(f, ",")?;
                }
                writeln!(f, "]")
            }
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Integer(arg0) => write!(f, "Integer: {arg0}"),
            Self::Scalar(arg0) => write!(f, "Scalar: {arg0}"),
            Self::Length(arg0) => write!(f, "Length: {arg0}"),
            Self::Area(arg0) => write!(f, "Area: {arg0}"),
            Self::Volume(arg0) => write!(f, "Volume: {arg0}"),
            Self::Vec2(arg0) => write!(f, "Vec2: {arg0:?}"),
            Self::Vec3(arg0) => write!(f, "Vec3: {arg0:?}"),
            Self::Vec4(arg0) => write!(f, "Vec4: {arg0:?}"),
            Self::Angle(arg0) => write!(f, "Angle: {arg0}"),
            Self::Weight(arg0) => write!(f, "Weight: {arg0}"),
            Self::Bool(arg0) => write!(f, "Bool: {arg0}"),
            Self::String(arg0) => write!(f, "String: {arg0}"),
            Self::Color(arg0) => write!(f, "Color: {arg0}"),
            Self::List(arg0) => write!(f, "List: {arg0}"),
            Self::Map(arg0) => write!(f, "Map: {arg0}"),
            Self::NamedTuple(arg0) => write!(f, "NamedTuple: {arg0}"),
            Self::UnnamedTuple(arg0) => write!(f, "UnnamedTuple: {arg0}"),
            Self::Node(arg0) => write!(f, "Node: {arg0}"),
            Self::NodeMultiplicity(arg0) => write!(f, "NodeMultiplicity: {arg0:?}"),
        }
    }
}

macro_rules! impl_try_from {
    ($($variant:ident),+ => $ty:ty ) => {
        impl TryFrom<Value> for $ty {
            type Error = ValueError;

            fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
                match value {
                    $(Value::$variant(v) => Ok(v),)*
                    value => Err(ValueError::CannotConvert(value, stringify!($ty).into())),
                }
            }
        }

        impl TryFrom<&Value> for $ty {
            type Error = ValueError;

            fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
                match value {
                    $(Value::$variant(v) => Ok(v.clone().into()),)*
                    value => Err(ValueError::CannotConvert(value.clone(), stringify!($ty).into())),
                }
            }
        }
    };
}

impl_try_from!(Integer => i64);
impl_try_from!(Scalar, Length, Angle => Scalar);
impl_try_from!(Vec2 => Vec2);
impl_try_from!(Vec3 => Vec3);
impl_try_from!(Vec4 => Vec4);
impl_try_from!(Bool => bool);
impl_try_from!(String => String);
impl_try_from!(Color => Color);

impl From<Vec<ObjectNode>> for Value {
    fn from(nodes: Vec<ObjectNode>) -> Self {
        match nodes.len() {
            0 => Value::None,
            1 => Value::Node(nodes.first().expect("Node").clone()),
            _ => Value::NodeMultiplicity(nodes),
        }
    }
}

#[cfg(test)]
fn integer(value: i64) -> Value {
    Value::Integer(value)
}

#[cfg(test)]
fn scalar(value: f64) -> Value {
    Value::Scalar(value)
}

#[cfg(test)]
fn check(result: ValueResult, value: Value) {
    let result = result.expect("error result");
    assert_eq!(result, value);
}

#[test]
fn test_value_integer() {
    let u = || integer(2);
    let v = || integer(5);
    let w = || scalar(5.0);

    // symmetric operations
    check(u() + v(), integer(2 + 5));
    check(u() - v(), integer(2 - 5));
    check(u() * v(), integer(2 * 5));
    check(u() / v(), scalar(2.0 / 5.0));
    check(-u(), integer(-2));

    // asymmetric operations
    check(u() + w(), scalar(2 as Scalar + 5.0));
    check(u() - w(), scalar(2 as Scalar - 5.0));
    check(u() * w(), scalar(2 as Scalar * 5.0));
    check(u() / w(), scalar(2.0 / 5.0));
}

#[test]
fn test_value_scalar() {
    let u = || scalar(2.0);
    let v = || scalar(5.0);
    let w = || integer(5);

    // symmetric operations
    check(u() + v(), scalar(2.0 + 5.0));
    check(u() - v(), scalar(2.0 - 5.0));
    check(u() * v(), scalar(2.0 * 5.0));
    check(u() / v(), scalar(2.0 / 5.0));
    check(-u(), scalar(-2.0));

    // asymmetric operations
    check(u() + w(), scalar(2.0 + 5.0));
    check(u() - w(), scalar(2.0 - 5.0));
    check(u() * w(), scalar(2.0 * 5.0));
    check(u() / w(), scalar(2.0 / 5.0));
}

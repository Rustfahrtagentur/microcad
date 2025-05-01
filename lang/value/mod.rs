// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad value related evaluation entities

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

pub(crate) type ValueResult = std::result::Result<Value, ValueError>;

/// A variant value
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Invalid value (used for error handling)
    None,
    /// An integer value
    Integer(Refer<Integer>),
    /// A scalar value
    Scalar(Refer<Scalar>),
    /// Length in mm
    Length(Refer<Scalar>),
    /// Area in mm²
    Area(Refer<Scalar>),
    /// Volume in mm³
    Volume(Refer<Scalar>),
    /// A 2D vector with length
    Vec2(Refer<Vec2>),
    /// A 3D vector with length
    Vec3(Refer<Vec3>),
    /// A 4D vector with length
    Vec4(Refer<Vec4>),
    /// An angle in radians
    Angle(Refer<Scalar>),
    /// Weight of a specific volume of material
    Weight(Refer<Scalar>),
    /// Boolean value
    Bool(Refer<bool>),
    /// String value
    String(Refer<String>),
    /// Color value
    Color(Refer<Color>),
    /// List
    List(List),
    /// Hash Map
    Map(Map),
    /// Tuple of named items
    NamedTuple(NamedTuple),
    /// Tuple of unnamed items
    UnnamedTuple(UnnamedTuple),
    /// A node in the render tree
    Node(ObjectNode),
    /// A node list in the render tree, result from multiplicity
    NodeMultiplicity(Vec<ObjectNode>)
}

impl Value {
    /// Add a unit to a primitive value (Scalar or Integer)
    pub fn add_unit_to_unitless(&mut self, unit: Unit) -> std::result::Result<(), ValueError> {
        match (self.clone(), unit.ty()) {
            (Value::Integer(i), Type::Length) => {
                *self = Value::Length(Refer::new(unit.normalize(*i as Scalar), i.src_ref))
            }
            (Value::Integer(i), Type::Angle) => {
                *self = Value::Angle(Refer::new(unit.normalize(*i as Scalar), i.src_ref))
            }
            (Value::Scalar(s), Type::Length) => {
                *self = Value::Length(Refer::new(unit.normalize(*s), s.src_ref))
            }
            (Value::Scalar(s), Type::Angle) => {
                *self = Value::Angle(Refer::new(unit.normalize(*s), s.src_ref))
            }
            (value, _) => return Err(ValueError::CannotAddUnitToValueWithUnit(value.clone())),
        }
        Ok(())
    }

    /// Fetch nodes from this value
    pub fn fetch_nodes(self) -> Vec<ObjectNode> {
        match self {
            Self::Node(n) => vec![n],
            Self::NodeMultiplicity(n) => n,
            _ => vec![]
        }
    }

    /// Clone the value with a new source reference
    pub fn clone_with_src_ref(&self, src_ref: SrcRef) -> Self {
        match self {
            Value::None => Value::None,
            Value::Integer(i) => Value::Integer(Refer::new(i.value, src_ref)),
            Value::Scalar(s) => Value::Scalar(Refer::new(s.value, src_ref)),
            Value::Length(l) => Value::Length(Refer::new(l.value, src_ref)),
            Value::Area(a) => Value::Area(Refer::new(a.value, src_ref)),
            Value::Volume(v) => Value::Volume(Refer::new(v.value, src_ref)),
            Value::Vec2(v) => Value::Vec2(Refer::new(v.value, src_ref)),
            Value::Vec3(v) => Value::Vec3(Refer::new(v.value, src_ref)),
            Value::Vec4(v) => Value::Vec4(Refer::new(v.value, src_ref)),
            Value::Angle(a) => Value::Angle(Refer::new(a.value, src_ref)),
            Value::Weight(w) => Value::Weight(Refer::new(w.value, src_ref)),
            Value::Bool(b) => Value::Bool(Refer::new(b.value, src_ref)),
            Value::String(s) => Value::String(Refer::new(s.value.clone(), src_ref)),
            Value::Color(c) => Value::Color(Refer::new(c.value, src_ref)),
            Value::List(l) => Value::List(l.clone()),
            // Value::Map(m) => Value::Map(m.clone_with_src_ref(src_ref)),
            //Value::NamedTuple(t) => Value::NamedTuple(t.clone_with_src_ref(src_ref)),
            //Value::UnnamedTuple(t) => Value::UnnamedTuple(t.clone_with_src_ref(src_ref)),
            Value::Node(n) => Value::Node(n.clone()),
            _ => todo!("Implement Value::clone_with_src_ref for all variants"),
        }
    }

    /// Get property value for a value
    ///
    /// - `identifier`: Identifier of the property
    ///
    /// This function is used when accessing a property `p` of a value `v` with `p.v`.
    pub fn get_property_value(&self, identifier: &Identifier) -> Option<Value> {
        let get = |value| Some(Value::Scalar(Refer::none(value)));
        let id = identifier.id().as_str();
        match self {
            Value::Vec2(r) => match id {
                "x" => get(r.value.x),
                "y" => get(r.value.y),
                _ => None,
            },
            Value::Vec3(r) => match id {
                "x" => get(r.value.x),
                "y" => get(r.value.y),
                "z" => get(r.value.z),
                _ => None,
            },
            Value::Vec4(r) => match id {
                "x" => get(r.value.x),
                "y" => get(r.value.y),
                "z" => get(r.value.z),
                "w" => get(r.value.w),
                _ => None,
            },
            Value::Color(r) => match id {
                "r" => get(r.value.r as f64),
                "g" => get(r.value.g as f64),
                "b" => get(r.value.b as f64),
                "a" => get(r.value.a as f64),
                _ => None,
            },
            Value::NamedTuple(named_tuple) => named_tuple.get(identifier).cloned(),
            Value::Node(node) => node.borrow().get_property_value(identifier.id()).cloned(),
            _ => None,
        }
    }

    /// Check if the value is invalid
    pub fn is_invalid(&self) -> bool {
        matches!(self, Value::None)
    }

    /// Binary operation
    pub fn binary_op(lhs: Value, rhs: Value, op: &str) -> ValueResult {
        match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            "^" => unimplemented!(), // lhs.pow(&rhs),
            "&" => lhs & rhs,
            "|" => lhs | rhs,
            ">" => Ok(Value::Bool(Refer::new(lhs > rhs, SrcRef::merge(lhs, rhs)))),
            "<" => Ok(Value::Bool(Refer::new(lhs < rhs, SrcRef::merge(lhs, rhs)))),
            "≤" => Ok(Value::Bool(Refer::new(lhs <= rhs, SrcRef::merge(lhs, rhs)))),
            "≥" => Ok(Value::Bool(Refer::new(lhs >= rhs, SrcRef::merge(lhs, rhs)))),
            "~" => todo!("implement near ~="),
            "=" => Ok(Value::Bool(Refer::new(lhs == rhs, SrcRef::merge(lhs, rhs)))),
            "!=" => Ok(Value::Bool(Refer::new(lhs != rhs, SrcRef::merge(lhs, rhs)))),
            _ => unimplemented!("{op:?}"),
        }
    }

    /// Unary operation
    pub fn unary_op(self, op: &str) -> ValueResult {
        match op {
            "-" => -self,
            _ => unimplemented!(),
        }
    }
}

impl SrcReferrer for Value {
    fn src_ref(&self) -> SrcRef {
        match self {
            Value::None => SrcRef(None),
            Value::Integer(i) => i.src_ref(),
            Value::Scalar(s) => s.src_ref(),
            Value::Bool(b) => b.src_ref(),

            Value::Length(l) => l.src_ref(),
            Value::Angle(a) => a.src_ref(),
            Value::Weight(w) => w.src_ref(),

            Value::Area(a) => a.src_ref(),
            Value::Volume(v) => v.src_ref(),
            Value::Vec2(v) => v.src_ref(),
            Value::Vec3(v) => v.src_ref(),
            Value::Vec4(v) => v.src_ref(),
            Value::String(s) => s.src_ref(),
            Value::Color(c) => c.src_ref(),
            Value::List(list) => list.src_ref(),
            Value::Map(map) => map.src_ref(),
            Value::NamedTuple(named_tuple) => named_tuple.src_ref(),
            Value::UnnamedTuple(unnamed_tuple) => unnamed_tuple.src_ref(),
            Value::Node(_) | Value::NodeMultiplicity(_) => SrcRef(None),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Integer(lhs), Value::Integer(rhs)) => lhs.partial_cmp(rhs),
            (Value::Scalar(lhs), Value::Scalar(rhs))
            | (Value::Length(lhs), Value::Length(rhs))
            | (Value::Area(lhs), Value::Area(rhs))
            | (Value::Volume(lhs), Value::Volume(rhs))
            | (Value::Angle(lhs), Value::Angle(rhs))
            | (Value::Angle(lhs), Value::Scalar(rhs))
            | (Value::Scalar(lhs), Value::Angle(rhs)) => lhs.partial_cmp(rhs),
            (Value::Vec2(lhs), Value::Vec2(rhs)) => lhs.magnitude2().partial_cmp(&rhs.magnitude2()),
            (Value::Vec3(lhs), Value::Vec3(rhs)) => lhs.magnitude2().partial_cmp(&rhs.magnitude2()),
            _ => None,
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
            Value::Integer(n) => Ok(Value::Integer(-n.clone())),
            Value::Scalar(n) => Ok(Value::Scalar(-n.clone())),
            Value::Length(n) => Ok(Value::Length(-n.clone())),
            Value::Vec2(v) => Ok(Value::Vec2(-v.clone())),
            Value::Vec3(v) => Ok(Value::Vec3(-v.clone())),
            Value::Angle(n) => Ok(Value::Angle(-n.clone())),
            _ => Err(ValueError::InvalidOperator("-".into())),
        }
    }
}

/// Rules for operator +
impl std::ops::Add for Value {
    type Output = ValueResult;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Add two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs + rhs)),
            // Add a scalar to an integer
            (Value::Integer(lhs), Value::Scalar(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l as Scalar + r
                })))
            }
            // Add an integer to a scalar
            (Value::Scalar(lhs), Value::Integer(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l + r as Scalar
                })))
            }
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
            (Value::String(lhs), Value::String(rhs)) => {
                Ok(Value::String(Refer::merge(lhs, rhs, |l, r| l + &r)))
            }
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
                    SrcRef::merge(lhs, rhs),
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

/// Rules for operator -
impl std::ops::Sub for Value {
    type Output = ValueResult;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Subtract two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs - rhs)),
            // Subtract an scalar and an integer
            (Value::Scalar(lhs), Value::Integer(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l - r as Scalar
                })))
            }
            // Subtract an integer and a scalar
            (Value::Integer(lhs), Value::Scalar(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l as Scalar - r
                })))
            }
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

/// Rules for operator *
impl std::ops::Mul for Value {
    type Output = ValueResult;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Multiply two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs * rhs)),
            // Multiply an integer and a scalar, result is scalar
            (Value::Integer(lhs), Value::Scalar(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l as Scalar * r
                })))
            }
            // Multiply a scalar and an integer, result is scalar
            (Value::Scalar(lhs), Value::Integer(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l * r as Scalar
                })))
            }
            // Multiply two scalars
            (Value::Scalar(lhs), Value::Scalar(rhs)) => Ok(Value::Scalar(lhs * rhs)),
            // Scale an angle with a scalar
            (Value::Angle(lhs), Value::Scalar(rhs)) | (Value::Scalar(rhs), Value::Angle(lhs)) => {
                Ok(Value::Angle(lhs * rhs))
            }
            // Scale an angle with an integer
            (Value::Angle(lhs), Value::Integer(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l * r as Scalar
                })))
            }
            // Scale an integer with an angle
            (Value::Integer(lhs), Value::Angle(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l as Scalar * r
                })))
            }
            // Scale a length with a scalar
            (Value::Length(lhs), Value::Scalar(rhs)) | (Value::Scalar(rhs), Value::Length(lhs)) => {
                Ok(Value::Length(lhs * rhs))
            }
            // Scale a length with an integer
            (Value::Length(lhs), Value::Integer(rhs)) => {
                Ok(Value::Length(Refer::merge(lhs, rhs, |l, r| {
                    l * r as Scalar
                })))
            }
            // Scale an integer with a length
            (Value::Integer(lhs), Value::Length(rhs)) => {
                Ok(Value::Length(Refer::merge(lhs, rhs, |l, r| {
                    l as Scalar * r
                })))
            }
            // Scale Vec2
            (Value::Scalar(lhs), Value::Vec2(rhs)) | (Value::Vec2(rhs), Value::Scalar(lhs)) => {
                Ok(Value::Vec2(Refer::merge(lhs, rhs, |l, r| {
                    Vec2::new(l * r.x, l * r.y)
                })))
            }
            // Scale Vec3
            (Value::Scalar(lhs), Value::Vec3(rhs)) | (Value::Vec3(rhs), Value::Scalar(lhs)) => {
                Ok(Value::Vec3(Refer::merge(lhs, rhs, |l, r| {
                    Vec3::new(l * r.x, l * r.y, l * r.z)
                })))
            }
            (Value::List(list), value) | (value, Value::List(list)) => Ok(list * value)?,
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} * {rhs}"))),
        }
    }
}

/// Rules for operator /
impl std::ops::Div for Value {
    type Output = ValueResult;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Division with scalar result
            (Value::Integer(lhs), Value::Integer(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l as Scalar / r as Scalar
                })))
            }
            (Value::Scalar(lhs), Value::Integer(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l / r as Scalar
                })))
            }
            (Value::Integer(lhs), Value::Scalar(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l as Scalar / r
                })))
            }
            (Value::Scalar(lhs), Value::Scalar(rhs))
            | (Value::Length(lhs), Value::Length(rhs))
            | (Value::Angle(lhs), Value::Angle(rhs)) => Ok(Value::Scalar(lhs / rhs)),
            (Value::Length(lhs), Value::Scalar(rhs)) => Ok(Value::Length(lhs / rhs)),
            (Value::Angle(lhs), Value::Scalar(rhs)) => Ok(Value::Angle(lhs / rhs)),
            (Value::Length(lhs), Value::Integer(rhs)) => {
                Ok(Value::Length(Refer::merge(lhs, rhs, |l, r| {
                    l / r as Scalar
                })))
            }
            (Value::Angle(lhs), Value::Integer(rhs)) => {
                Ok(Value::Angle(Refer::merge(lhs, rhs, |l, r| l / r as Scalar)))
            }
            (Value::List(list), value) => Ok(list / value)?,
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} / {rhs}"))),
        }
    }
}

/// Rules for operator | (union operator)
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

/// Rules for operator & (intersection operator)
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

macro_rules! impl_try_from {
    ($($variant:ident),+ => $ty:ty ) => {
        impl TryFrom<Value> for $ty {
            type Error = ValueError;

            fn try_from(value: Value) -> std::result::Result<Self, Self::Error> {
                match value {
                    $(Value::$variant(v) => Ok(v.value.into()),)*
                    value => Err(ValueError::CannotConvert(value, stringify!($ty).into())),
                }
            }
        }

        impl TryFrom<&Value> for $ty {
            type Error = ValueError;

            fn try_from(value: &Value) -> std::result::Result<Self, Self::Error> {
                match value {
                    $(Value::$variant(v) => Ok(v.value.clone().into()),)*
                    value => Err(ValueError::CannotConvert(value.clone(), stringify!($ty).into())),
                }
            }
        }
    };
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(Refer::none(s))
    }
}

impl From<Scalar> for Value {
    fn from(value: Scalar) -> Self {
        Value::Scalar(Refer::none(value))
    }
}

impl_try_from!(Integer => i64);
impl_try_from!(Scalar, Length, Angle => Scalar);
impl_try_from!(Vec2 => Vec2);
impl_try_from!(Vec3 => Vec3);
impl_try_from!(Vec4 => Vec4);
impl_try_from!(Bool => bool);
impl_try_from!(String => String);
impl_try_from!(Color => Color);

#[cfg(test)]
fn integer(value: i64, src_ref: &SrcRef) -> Value {
    Value::Integer(Refer::new(value, src_ref.clone()))
}

#[cfg(test)]
fn scalar(value: f64, src_ref: &SrcRef) -> Value {
    Value::Scalar(Refer::new(value, src_ref.clone()))
}

#[cfg(test)]
fn check(result: ValueResult, value: Value) {
    let result = result.expect("error result");
    assert_eq!(result, value);
    // SrcRef cannot be compared with PartialEq
    assert_eq!(result.src_ref().to_string(), value.src_ref().to_string());
}

#[test]
fn test_value_integer() {
    crate::env_logger_init();

    let u = || integer(2, &SrcRef::new(3..4, 5, 6, 0));
    let v = || integer(5, &SrcRef::new(6..7, 8, 9, 0));
    let w = || scalar(5.0, &SrcRef::new(6..7, 8, 9, 0));

    let r = SrcRef::new(3..7, 5, 6, 0);

    // symmetric operations
    check(u() + v(), integer(2 + 5, &r));
    check(u() - v(), integer(2 - 5, &r));
    check(u() * v(), integer(2 * 5, &r));
    check(u() / v(), scalar(2.0 / 5.0, &r));
    check(-u(), integer(-2, &r));

    // asymmetric operations
    check(u() + w(), scalar(2 as Scalar + 5.0, &r));
    check(u() - w(), scalar(2 as Scalar - 5.0, &r));
    check(u() * w(), scalar(2 as Scalar * 5.0, &r));
    check(u() / w(), scalar(2.0 / 5.0, &r));
}

#[test]
fn test_value_scalar() {
    crate::env_logger_init();

    let u = || scalar(2.0, &SrcRef::new(3..4, 5, 6, 0));
    let v = || scalar(5.0, &SrcRef::new(6..7, 8, 9, 0));
    let w = || integer(5, &SrcRef::new(6..7, 8, 9, 0));

    let r = SrcRef::new(3..7, 5, 6, 0);

    // symmetric operations
    check(u() + v(), scalar(2.0 + 5.0, &r));
    check(u() - v(), scalar(2.0 - 5.0, &r));
    check(u() * v(), scalar(2.0 * 5.0, &r));
    check(u() / v(), scalar(2.0 / 5.0, &r));
    check(-u(), scalar(-2.0, &r));

    // asymmetric operations
    check(u() + w(), scalar(2.0 + 5.0, &r));
    check(u() - w(), scalar(2.0 - 5.0, &r));
    check(u() * w(), scalar(2.0 * 5.0, &r));
    check(u() / w(), scalar(2.0 / 5.0, &r));
}

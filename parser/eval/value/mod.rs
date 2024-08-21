mod error;
mod list;
mod map;
mod map_key_value;
mod named_tuple;
mod unnamed_tuple;
mod value_list;

pub use error::*;
pub use list::*;
pub use map::*;
pub use map_key_value::*;
pub use named_tuple::*;
pub use unnamed_tuple::*;
pub use value_list::*;

use crate::{eval::*, language::*, r#type::*};
use cgmath::InnerSpace;
use microcad_core::*;
use microcad_render::tree::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// An integer value
    Integer(i64),
    /// A scalar value
    Scalar(Scalar),
    /// Length in mm
    Length(Scalar),
    /// A 2D vector with length
    Vec2(Vec2),
    /// A 3D vector with length
    Vec3(Vec3),
    /// A 4D vector with length
    Vec4(Vec4),
    /// An angle in radians
    Angle(Scalar),
    /// Boolean value
    Bool(bool),
    /// String value
    String(String),
    /// Color value
    Color(Color),
    // List
    List(List),
    // Hash Map
    Map(Map),
    /// Tuple of named items
    NamedTuple(NamedTuple),
    /// Tuple of unnamed items
    UnnamedTuple(UnnamedTuple),
    /// A node in the render tree
    Node(Node),
}

impl Value {
    pub fn less_than(&self, rhs: &Self) -> Result<bool, ValueError> {
        match (self, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(lhs < rhs),
            (Value::Scalar(lhs), Value::Scalar(rhs)) => Ok(lhs < rhs),
            (Value::Length(lhs), Value::Length(rhs)) => Ok(lhs < rhs),
            (Value::Vec2(lhs), Value::Vec2(rhs)) => Ok(lhs.magnitude2() < rhs.magnitude2()),
            (Value::Vec3(lhs), Value::Vec3(rhs)) => Ok(lhs.magnitude2() < rhs.magnitude2()),
            (Value::Angle(lhs), Value::Angle(rhs)) => Ok(lhs < rhs),
            _ => Err(ValueError::InvalidOperator('<')),
        }
    }

    pub fn greater_than(&self, rhs: &Self) -> Result<bool, ValueError> {
        match (self, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(lhs > rhs),
            (Value::Scalar(lhs), Value::Scalar(rhs)) => Ok(lhs > rhs),
            (Value::Length(lhs), Value::Length(rhs)) => Ok(lhs > rhs),
            (Value::Vec2(lhs), Value::Vec2(rhs)) => Ok(lhs.magnitude2() > rhs.magnitude2()),
            (Value::Vec3(lhs), Value::Vec3(rhs)) => Ok(lhs.magnitude2() > rhs.magnitude2()),
            (Value::Angle(lhs), Value::Angle(rhs)) => Ok(lhs > rhs),
            _ => Err(ValueError::InvalidOperator('>')),
        }
    }

    pub fn less_than_or_equal(&self, rhs: &Self) -> Result<bool, ValueError> {
        Ok(self.less_than(rhs)? || self.eq(rhs))
    }

    pub fn greater_than_or_equal(&self, rhs: &Self) -> Result<bool, ValueError> {
        Ok(self.greater_than(rhs)? || self.eq(rhs))
    }

    pub fn neg(&self) -> Result<Value, ValueError> {
        match self {
            Value::Integer(n) => Ok(Value::Integer(-n)),
            Value::Scalar(n) => Ok(Value::Scalar(-n)),
            Value::Length(n) => Ok(Value::Length(-n)),
            Value::Vec2(v) => Ok(Value::Vec2(-*v)),
            Value::Vec3(v) => Ok(Value::Vec3(-*v)),
            Value::Angle(n) => Ok(Value::Angle(-n)),
            _ => Err(ValueError::InvalidOperator('-')),
        }
    }

    /// Add a unit to a scalar value
    pub fn add_unit_to_unitless_types(&mut self, unit: Unit) -> Result<(), ValueError> {
        match (self.clone(), unit.ty()) {
            (Value::Integer(i), Type::Length) => *self = Value::Length(unit.normalize(i as Scalar)),
            (Value::Integer(i), Type::Angle) => *self = Value::Angle(unit.normalize(i as Scalar)),
            (Value::Scalar(s), Type::Length) => *self = Value::Length(unit.normalize(s)),
            (Value::Scalar(s), Type::Angle) => *self = Value::Angle(unit.normalize(s)),
            (value, _) => return Err(ValueError::CannotAddUnitToValueWithUnit(value.clone())),
        }
        Ok(())
    }
}

impl Ty for Value {
    fn ty(&self) -> Type {
        match self {
            Value::Integer(_) => Type::Integer,
            Value::Scalar(_) => Type::Scalar,
            Value::Length(_) => Type::Length,
            Value::Vec2(_) => Type::Vec2,
            Value::Vec3(_) => Type::Vec3,
            Value::Vec4(_) => Type::Vec4,
            Value::Angle(_) => Type::Angle,
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Color(_) => Type::Color,
            Value::List(list) => list.ty(),
            Value::Map(map) => map.ty(),
            Value::NamedTuple(named_tuple) => named_tuple.ty(),
            Value::UnnamedTuple(unnamed_tuple) => unnamed_tuple.ty(),
            Value::Node(_) => Type::Node,
        }
    }
}

/// Rules for operator +
impl std::ops::Add for Value {
    type Output = Result<Value, ValueError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Add two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs + rhs)),
            // Add an integer and a scalar
            (Value::Integer(lhs), Value::Scalar(rhs))
            | (Value::Scalar(rhs), Value::Integer(lhs)) => Ok(Value::Scalar(lhs as Scalar + rhs)),
            // Add two scalars
            (Value::Scalar(lhs), Value::Scalar(rhs)) => Ok(Value::Scalar(lhs + rhs)),
            // Add two angles
            (Value::Angle(lhs), Value::Angle(rhs)) => Ok(Value::Angle(lhs + rhs)),
            // Add two lengths
            (Value::Length(lhs), Value::Length(rhs)) => Ok(Value::Angle(lhs + rhs)),
            // Add two Vec2
            (Value::Vec2(lhs), Value::Vec2(rhs)) => Ok(Value::Vec2(lhs + rhs)),
            // Add two Vec3
            (Value::Vec3(lhs), Value::Vec3(rhs)) => Ok(Value::Vec3(lhs + rhs)),
            // Concatenate two strings
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::String(lhs + &rhs)),
            // Concatenate two lists
            (Value::List(mut lhs), Value::List(mut rhs)) => {
                lhs.append(&mut rhs);
                Ok(Value::List(lhs))
            }
            // Add values of two tuples of the same length
            (Value::UnnamedTuple(lhs), Value::UnnamedTuple(rhs)) => {
                Ok(Value::UnnamedTuple((lhs + rhs)?))
            }
            _ => Err(ValueError::InvalidOperator('+')),
        }
    }
}

/// Rules for operator -
impl std::ops::Sub for Value {
    type Output = Result<Value, ValueError>;

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
            (Value::Length(lhs), Value::Length(rhs)) => Ok(Value::Angle(lhs - rhs)),
            // Subtract two Vec2
            (Value::Vec2(lhs), Value::Vec2(rhs)) => Ok(Value::Vec2(lhs - rhs)),
            // Subtract two Vec3
            (Value::Vec3(lhs), Value::Vec3(rhs)) => Ok(Value::Vec3(lhs - rhs)),
            // Remove an elements from list `rhs` from list `lhs`
            (Value::List(mut lhs), Value::List(rhs)) => {
                lhs.retain(|x| !rhs.contains(x));
                Ok(Value::List(lhs))
            }
            // Subtract values of two arrays of the same length
            (Value::UnnamedTuple(lhs), Value::UnnamedTuple(rhs)) => {
                Ok(Value::UnnamedTuple((lhs - rhs)?))
            }
            _ => Err(ValueError::InvalidOperator('-')),
        }
    }
}

/// Rules for operator *
impl std::ops::Mul for Value {
    type Output = Result<Value, ValueError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Multiply two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs * rhs)),
            // Multiply an integer and a scalar
            (Value::Integer(lhs), Value::Scalar(rhs))
            | (Value::Scalar(rhs), Value::Integer(lhs)) => Ok(Value::Scalar(lhs as Scalar * rhs)),
            // Multiply two scalars
            (Value::Scalar(lhs), Value::Scalar(rhs)) => Ok(Value::Scalar(lhs * rhs)),
            // Scale an angle
            (Value::Angle(lhs), Value::Scalar(rhs)) | (Value::Scalar(rhs), Value::Angle(lhs)) => {
                Ok(Value::Angle(lhs * rhs))
            }
            // Scale a length
            (Value::Length(lhs), Value::Scalar(rhs)) | (Value::Scalar(rhs), Value::Length(lhs)) => {
                Ok(Value::Length(lhs * rhs))
            }
            // Scale Vec2
            (Value::Scalar(lhs), Value::Vec2(rhs)) | (Value::Vec2(rhs), Value::Scalar(lhs)) => {
                Ok(Value::Vec2(Vec2::new(lhs * rhs.x, lhs * rhs.y)))
            }
            // Scale Vec3
            (Value::Scalar(lhs), Value::Vec3(rhs)) | (Value::Vec3(rhs), Value::Scalar(lhs)) => Ok(
                Value::Vec3(Vec3::new(lhs * rhs.x, lhs * rhs.y, lhs * rhs.z)),
            ),
            _ => Err(ValueError::InvalidOperator('*')),
        }
    }
}

/// Rules for operator /
impl std::ops::Div for Value {
    type Output = Result<Value, ValueError>;

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
            _ => Err(ValueError::InvalidOperator('/')),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Scalar(n) => write!(f, "{}", n),
            Value::Length(n) | Value::Angle(n) => write!(f, "{}{}", n, self.ty().default_unit()),
            Value::Vec2(v) => write!(f, "({}, {})", v.x, v.y),
            Value::Vec3(v) => write!(f, "({}, {}, {})", v.x, v.y, v.z),
            Value::Vec4(v) => write!(f, "({}, {}, {}, {})", v.x, v.y, v.z, v.w),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Color(c) => write!(f, "{}", c),
            Value::List(l) => write!(f, "{}", l),
            Value::Map(m) => write!(f, "{}", m),
            Value::NamedTuple(t) => write!(f, "{}", t),
            Value::UnnamedTuple(t) => write!(f, "{}", t),
            Value::Node(n) => write!(f, "{:?}", n),
        }
    }
}

macro_rules! impl_try_from {
    ($($variant:ident),+ => $ty:ty ) => {
        impl TryFrom<Value> for $ty {
            type Error = ValueError;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                match value {
                    $(Value::$variant(v) => Ok(v.into()),)*
                    value => Err(ValueError::CannotConvert(value, stringify!($ty).into())),
                }
            }
        }

        impl TryFrom<&Value> for $ty {
            type Error = ValueError;

            fn try_from(value: &Value) -> Result<Self, Self::Error> {
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

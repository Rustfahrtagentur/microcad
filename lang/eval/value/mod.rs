mod error;
mod list;
mod map;
mod map_key_value;
mod named_tuple;
mod unnamed_tuple;
mod value_list;

pub use error::ValueError;
pub use list::*;
pub use map::*;
pub use map_key_value::*;
pub use named_tuple::*;
pub use unnamed_tuple::*;
pub use value_list::*;

use crate::{eval::*, parse::*, r#type::*, src_ref::*};
use cgmath::InnerSpace;
use microcad_core::*;
use microcad_render::tree::Node;

pub(crate) type ValueResult = std::result::Result<Value, ValueError>;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// An integer value
    Integer(Refer<Integer>),
    /// A scalar value
    Scalar(Refer<Scalar>),
    /// Length in mm
    Length(Refer<Scalar>),
    /// A 2D vector with length
    Vec2(Refer<Vec2>),
    /// A 3D vector with length
    Vec3(Refer<Vec3>),
    /// A 4D vector with length
    Vec4(Refer<Vec4>),
    /// An angle in radians
    Angle(Refer<Scalar>),
    /// Boolean value
    Bool(Refer<bool>),
    /// String value
    String(Refer<String>),
    /// Color value
    Color(Refer<Color>),
    // List
    List(Refer<List>),
    // Hash Map
    Map(Refer<Map>),
    /// Tuple of named items
    NamedTuple(Refer<NamedTuple>),
    /// Tuple of unnamed items
    UnnamedTuple(Refer<UnnamedTuple>),
    /// A node in the render tree
    Node(Node),
}

impl Value {
    pub fn less_than(&self, rhs: &Self) -> std::result::Result<bool, ValueError> {
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

    pub fn greater_than(&self, rhs: &Self) -> std::result::Result<bool, ValueError> {
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

    pub fn less_than_or_equal(&self, rhs: &Self) -> std::result::Result<bool, ValueError> {
        Ok(self.less_than(rhs)? || self.eq(rhs))
    }

    pub fn greater_than_or_equal(&self, rhs: &Self) -> std::result::Result<bool, ValueError> {
        Ok(self.greater_than(rhs)? || self.eq(rhs))
    }

    pub fn neg(&self) -> ValueResult {
        match self {
            Value::Integer(n) => Ok(Value::Integer(-n.clone())),
            Value::Scalar(n) => Ok(Value::Scalar(-n.clone())),
            Value::Length(n) => Ok(Value::Length(-n.clone())),
            Value::Vec2(v) => Ok(Value::Vec2(-v.clone())),
            Value::Vec3(v) => Ok(Value::Vec3(-v.clone())),
            Value::Angle(n) => Ok(Value::Angle(-n.clone())),
            _ => Err(ValueError::InvalidOperator('-')),
        }
    }

    /// Add a unit to a scalar value
    pub fn add_unit_to_unitless_types(
        &mut self,
        unit: Unit,
    ) -> std::result::Result<(), ValueError> {
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
}

impl SrcReferrer for Value {
    fn src_ref(&self) -> SrcRef {
        match self {
            Value::Integer(i) => i.src_ref.clone(),
            Value::Scalar(s) => s.src_ref.clone(),
            Value::Length(l) => l.src_ref.clone(),
            Value::Vec2(v) => v.src_ref.clone(),
            Value::Vec3(v) => v.src_ref.clone(),
            Value::Vec4(v) => v.src_ref.clone(),
            Value::Angle(a) => a.src_ref.clone(),
            Value::Bool(b) => b.src_ref.clone(),
            Value::String(s) => s.src_ref.clone(),
            Value::Color(c) => c.src_ref.clone(),
            Value::List(list) => list.src_ref.clone(),
            Value::Map(map) => map.src_ref.clone(),
            Value::NamedTuple(named_tuple) => named_tuple.src_ref.clone(),
            Value::UnnamedTuple(unnamed_tuple) => unnamed_tuple.src_ref.clone(),
            Value::Node(_) => SrcRef(None),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Integer(lhs), Value::Integer(rhs)) => lhs.partial_cmp(rhs),
            (Value::Scalar(lhs), Value::Scalar(rhs)) => lhs.partial_cmp(rhs),
            (Value::Length(lhs), Value::Length(rhs)) => lhs.partial_cmp(rhs),
            (Value::Vec2(lhs), Value::Vec2(rhs)) => lhs.magnitude2().partial_cmp(&rhs.magnitude2()),
            (Value::Vec3(lhs), Value::Vec3(rhs)) => lhs.magnitude2().partial_cmp(&rhs.magnitude2()),
            (Value::Angle(lhs), Value::Angle(rhs)) => lhs.partial_cmp(rhs),
            _ => None,
        }
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
    type Output = ValueResult;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Add two integers
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs + rhs)),
            // Add a scalar to an integer
            (Value::Integer(lhs), Value::Scalar(rhs)) => {
                Ok(Value::Integer(Refer::merge(lhs, rhs, |l, r| {
                    l + r as Integer
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
            (Value::Length(lhs), Value::Length(rhs)) => Ok(Value::Angle(lhs + rhs)),
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
                if lhs.value.ty() == rhs.value.ty() {
                    let res = lhs.value.iter().chain(rhs.value.iter());
                    Ok(Value::List(Refer::new(
                        List::new(res.cloned().collect(), lhs.value.ty()),
                        SrcRef::merge(lhs.src_ref, rhs.src_ref),
                    )))
                } else {
                    Err(ValueError::CannotCombineVecOfDifferentType(
                        lhs.value.ty(),
                        rhs.value.ty(),
                    ))
                }
            }
            // Add values of two tuples of the same length
            (Value::UnnamedTuple(lhs), Value::UnnamedTuple(rhs)) => {
                Ok(Value::UnnamedTuple(Refer::new(
                    (lhs.value + rhs.value)?,
                    SrcRef::merge(lhs.src_ref, rhs.src_ref),
                )))
            }
            _ => Err(ValueError::InvalidOperator('+')),
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
                Ok(Value::Integer(Refer::merge(lhs, rhs, |l, r| {
                    l - r as Integer
                })))
            }
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
                if lhs.value.ty() == rhs.value.ty() {
                    lhs.retain(|x| !rhs.contains(x));
                    Ok(Value::List(lhs))
                } else {
                    Err(ValueError::CannotCombineVecOfDifferentType(
                        lhs.value.ty(),
                        rhs.value.ty(),
                    ))
                }
            }
            // Subtract values of two arrays of the same length
            (Value::UnnamedTuple(lhs), Value::UnnamedTuple(rhs)) => {
                Ok(Value::UnnamedTuple(Refer::new(
                    (lhs.value - rhs.value)?,
                    SrcRef::merge(lhs.src_ref, rhs.src_ref),
                )))
            }
            _ => Err(ValueError::InvalidOperator('-')),
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
            // Multiply an integer and a scalar
            (Value::Integer(lhs), Value::Scalar(rhs)) => {
                Ok(Value::Integer(Refer::merge(lhs, rhs, |l, r| {
                    l * r as Integer
                })))
            }
            (Value::Scalar(lhs), Value::Integer(rhs)) => {
                Ok(Value::Scalar(Refer::merge(lhs, rhs, |l, r| {
                    l * r as Scalar
                })))
            }
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
            _ => Err(ValueError::InvalidOperator('*')),
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

impl_try_from!(Integer => i64);
impl_try_from!(Scalar, Length, Angle => Scalar);
impl_try_from!(Vec2 => Vec2);
impl_try_from!(Vec3 => Vec3);
impl_try_from!(Vec4 => Vec4);
impl_try_from!(Bool => bool);
impl_try_from!(String => String);
impl_try_from!(Color => Color);

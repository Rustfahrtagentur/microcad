// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation entities.
//!
//! Every evaluation of any *symbol* leads to a [`Value`] which then might continued
//! to process or ends up as the overall evaluation result.

mod argument_value;
mod argument_value_list;
mod array;
mod matrix;
mod parameter_value;
mod parameter_value_list;
mod quantity;
mod tuple;
mod value_access;
mod value_error;
mod value_list;

pub use argument_value::*;
pub use argument_value_list::*;
pub use array::*;
pub use matrix::*;
pub use parameter_value::*;
pub use parameter_value_list::*;
pub use quantity::*;
pub use tuple::*;
pub use value_access::*;
pub use value_error::*;
pub use value_list::*;

use crate::{model::*, syntax::*, ty::*, *};
use microcad_core::*;

pub(crate) type ValueResult<Type = Value> = std::result::Result<Type, ValueError>;

/// A variant value with attached source code reference.
#[derive(Clone, Default, PartialEq)]
pub enum Value {
    /// Invalid value (used for error handling).
    #[default]
    None,
    /// A quantity value.
    Quantity(Quantity),
    /// A boolean value.
    Bool(bool),
    /// An integer value.
    Integer(Integer),
    /// A string value.
    String(String),
    /// A list of values with a common type.
    Array(Array),
    /// A tuple of named items.
    Tuple(Box<Tuple>),
    /// A matrix.
    Matrix(Box<Matrix>),
    /// A model in the model tree.
    Model(Model),
    /// Return value
    Return(Box<Value>),
    /// Direction (X,Y or Z)
    Direction(Direction),
    /// Alignment
    Alignment(Alignment),
}

impl Value {
    /// Check if the value is invalid.
    pub fn is_invalid(&self) -> bool {
        matches!(self, Value::None)
    }

    /// Calculate the power of two values, if possible.
    pub fn pow(&self, rhs: &Value) -> ValueResult {
        match (&self, rhs) {
            (Value::Quantity(lhs), Value::Quantity(rhs)) => Ok(Value::Quantity(lhs.pow(rhs))),
            (Value::Quantity(lhs), Value::Integer(rhs)) => Ok(Value::Quantity(lhs.pow_int(rhs))),
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Integer(lhs.pow(*rhs as u32))),
            _ => Err(ValueError::InvalidOperator("^".to_string())),
        }
    }

    /// Binary operation
    pub fn binary_op(lhs: Value, rhs: Value, op: &str) -> ValueResult {
        match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs / rhs,
            "^" => lhs.pow(&rhs),
            "&" => lhs & rhs,
            "|" => lhs | rhs,
            ">" => Ok(Value::Bool(lhs > rhs)),
            "<" => Ok(Value::Bool(lhs < rhs)),
            "≤" => Ok(Value::Bool(lhs <= rhs)),
            "≥" => Ok(Value::Bool(lhs >= rhs)),
            "~" => todo!("implement near ~="),
            "==" => Ok(Value::Bool(lhs == rhs)),
            "!=" => Ok(Value::Bool(lhs != rhs)),
            _ => unimplemented!("{op:?}"),
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

    /// Try to convert to [`String`].
    pub fn try_string(&self) -> Result<String, ValueError> {
        match self {
            Value::String(s) => return Ok(s.clone()),
            Value::Integer(i) => return Ok(i.to_string()),
            _ => {}
        }

        Err(ValueError::CannotConvert(self.clone(), "String".into()))
    }

    /// Try to convert to [`Scalar`].
    pub fn try_scalar(&self) -> Result<Scalar, ValueError> {
        match self {
            Value::Quantity(q) => return Ok(q.value),
            Value::Integer(i) => return Ok((*i) as f64),
            _ => {}
        }

        Err(ValueError::CannotConvert(self.clone(), "Scalar".into()))
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // integer type
            (Value::Integer(lhs), Value::Integer(rhs)) => lhs.partial_cmp(rhs),
            (Value::Quantity(lhs), Value::Quantity(rhs)) => lhs.partial_cmp(rhs),
            (
                Value::Quantity(Quantity {
                    value,
                    quantity_type: QuantityType::Scalar,
                }),
                Value::Integer(rhs),
            ) => value.partial_cmp(&(*rhs as Scalar)),
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
            Value::Quantity(q) => q.ty(),
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Array(list) => list.ty(),
            Value::Tuple(tuple) => tuple.ty(),
            Value::Matrix(matrix) => matrix.ty(),
            Value::Model(_) => Type::Models,
            Value::Return(r) => r.ty(),
            Value::Direction(_) => Type::Direction,
            Value::Alignment(_) => Type::Alignment,
        }
    }
}

impl std::ops::Neg for Value {
    type Output = ValueResult;

    fn neg(self) -> Self::Output {
        match self {
            Value::Integer(n) => Ok(Value::Integer(-n)),
            Value::Quantity(q) => Ok(Value::Quantity(q.neg())),
            Value::Array(a) => -a,
            Value::Tuple(t) => -t.as_ref().clone(),
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
            // Add a quantity to an integer
            (Value::Integer(lhs), Value::Quantity(rhs)) => Ok(Value::Quantity((lhs + rhs)?)),
            // Add an integer to a quantity
            (Value::Quantity(lhs), Value::Integer(rhs)) => Ok(Value::Quantity((lhs + rhs)?)),
            // Add two scalars
            (Value::Quantity(lhs), Value::Quantity(rhs)) => Ok(Value::Quantity((lhs + rhs)?)),
            // Concatenate two strings
            (Value::String(lhs), Value::String(rhs)) => Ok(Value::String(lhs + &rhs)),
            // Concatenate two lists
            (Value::Array(lhs), Value::Array(rhs)) => {
                if lhs.ty() != rhs.ty() {
                    return Err(ValueError::CannotCombineVecOfDifferentType(
                        lhs.ty(),
                        rhs.ty(),
                    ));
                }

                Ok(Value::Array(Array::new(
                    lhs.iter().chain(rhs.iter()).cloned().collect(),
                    lhs.ty(),
                )))
            }
            // Add a value to an array.
            (Value::Array(lhs), rhs) => Ok((lhs + rhs)?),
            // Add two tuples of the same type: (x = 1., y = 2.) + (x = 3., y = 4.)
            (Value::Tuple(lhs), Value::Tuple(rhs)) => Ok((*lhs + *rhs)?.into()),
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
            (Value::Quantity(lhs), Value::Integer(rhs)) => Ok(Value::Quantity((lhs - rhs)?)),
            // Subtract an integer and a scalar
            (Value::Integer(lhs), Value::Quantity(rhs)) => Ok(Value::Quantity((lhs - rhs)?)),
            // Subtract two numbers
            (Value::Quantity(lhs), Value::Quantity(rhs)) => Ok(Value::Quantity((lhs - rhs)?)),
            // Subtract value to an array: `[1,2,3] - 1 = [0,1,2]`.
            (Value::Array(lhs), rhs) => Ok((lhs - rhs)?),
            // Subtract two tuples of the same type: (x = 1., y = 2.) - (x = 3., y = 4.)
            (Value::Tuple(lhs), Value::Tuple(rhs)) => Ok((*lhs - *rhs)?.into()),

            // Boolean difference operator for models
            (Value::Model(lhs), Value::Model(rhs)) => Ok(Value::Model(
                lhs.boolean_op(microcad_core::BooleanOp::Subtract, rhs),
            )),
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
            (Value::Integer(lhs), Value::Quantity(rhs)) => Ok(Value::Quantity((lhs * rhs)?)),
            // Multiply a scalar and an integer, result is scalar
            (Value::Quantity(lhs), Value::Integer(rhs)) => Ok(Value::Quantity((lhs * rhs)?)),
            // Multiply two scalars
            (Value::Quantity(lhs), Value::Quantity(rhs)) => Ok(Value::Quantity((lhs * rhs)?)),
            (Value::Array(array), value) | (value, Value::Array(array)) => Ok((array * value)?),
            (Value::Tuple(tuple), value) | (value, Value::Tuple(tuple)) => {
                Ok((tuple.as_ref().clone() * value)?.into())
            }
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} * {rhs}"))),
        }
    }
}

/// Multiply a Unit with a value. Used for unit bundling: `[1,2,3]mm`.
///
/// `[1,2,3]mm` is a shortcut for `[1,2,3] * 1mm`.
impl std::ops::Mul<Unit> for Value {
    type Output = ValueResult;

    fn mul(self, unit: Unit) -> Self::Output {
        match (self, unit.ty()) {
            (value, Type::Quantity(QuantityType::Scalar)) | (value, Type::Integer) => Ok(value),
            (Value::Integer(i), Type::Quantity(quantity_type)) => Ok(Value::Quantity(
                Quantity::new(unit.normalize(i as Scalar), quantity_type),
            )),
            (Value::Quantity(quantity), Type::Quantity(quantity_type)) => Ok(Value::Quantity(
                (quantity * Quantity::new(unit.normalize(1.0), quantity_type))?,
            )),
            (Value::Array(array), Type::Quantity(quantity_type)) => {
                Ok((array * Value::Quantity(Quantity::new(unit.normalize(1.0), quantity_type)))?)
            }
            (value, _) => Err(ValueError::CannotAddUnitToValueWithUnit(value.clone())),
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
                Ok(Value::Quantity((lhs as Scalar / rhs as Scalar).into()))
            }
            (Value::Quantity(lhs), Value::Integer(rhs)) => Ok(Value::Quantity((lhs / rhs)?)),
            (Value::Integer(lhs), Value::Quantity(rhs)) => Ok(Value::Quantity((lhs / rhs)?)),
            (Value::Quantity(lhs), Value::Quantity(rhs)) => Ok(Value::Quantity((lhs / rhs)?)),
            (Value::Array(array), value) => Ok((array / value)?),
            (Value::Tuple(tuple), value) => Ok((tuple.as_ref().clone() / value)?.into()),
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} / {rhs}"))),
        }
    }
}

/// Rules for operator `|`` (union).
impl std::ops::BitOr for Value {
    type Output = ValueResult;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Model(lhs), Value::Model(rhs)) => Ok(Value::Model(
                lhs.boolean_op(microcad_core::BooleanOp::Union, rhs),
            )),
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(lhs | rhs)),
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} | {rhs}"))),
        }
    }
}

/// Rules for operator `&` (intersection).
impl std::ops::BitAnd for Value {
    type Output = ValueResult;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Model(lhs), Value::Model(rhs)) => {
                Ok(Value::Model(lhs.boolean_op(BooleanOp::Intersect, rhs)))
            }
            (Value::Bool(lhs), Value::Bool(rhs)) => Ok(Value::Bool(lhs & rhs)),
            (lhs, rhs) => Err(ValueError::InvalidOperator(format!("{lhs} & {rhs}"))),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::None => write!(f, crate::invalid_no_ansi!(VALUE)),
            Value::Integer(n) => write!(f, "{n}"),
            Value::Quantity(q) => write!(f, "{q}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Array(l) => write!(f, "{l}"),
            Value::Tuple(t) => write!(f, "{t}"),
            Value::Matrix(m) => write!(f, "{m}"),
            Value::Model(n) => write!(f, "{n}"),
            Value::Return(r) => write!(f, "{r}"),
            Value::Direction(d) => write!(f, "{d}"),
            Value::Alignment(a) => write!(f, "{a}"),
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::None => write!(f, crate::invalid!(VALUE)),
            Value::Integer(n) => write!(f, "{n}"),
            Value::Quantity(q) => write!(f, "{q}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "\"{s}\""),
            Value::Array(l) => write!(f, "{l}"),
            Value::Tuple(t) => write!(f, "{t}"),
            Value::Matrix(m) => write!(f, "{m}"),
            Value::Model(n) => write!(f, "Models:\n {n}"),
            Value::Return(r) => write!(f, "Return: {r}"),
            Value::Direction(d) => write!(f, "{d}"),
            Value::Alignment(a) => write!(f, "{a}"),
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
impl_try_from!(Bool => bool);
impl_try_from!(String => String);

impl TryFrom<&Value> for Scalar {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => Ok(*i as Scalar),
            Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Scalar,
            }) => Ok(*value),
            _ => Err(ValueError::CannotConvert(value.clone(), "Scalar".into())),
        }
    }
}

impl TryFrom<Value> for Scalar {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(i) => Ok(i as Scalar),
            Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Scalar,
            }) => Ok(value),
            _ => Err(ValueError::CannotConvert(value.clone(), "Scalar".into())),
        }
    }
}

impl TryFrom<&Value> for Size2 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tuple(tuple) => Ok(tuple.as_ref().try_into()?),
            _ => Err(ValueError::CannotConvert(value.clone(), "Size2".into())),
        }
    }
}

impl TryFrom<&Value> for Mat3 {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Value::Matrix(m) = value {
            if let Matrix::Matrix3(matrix3) = m.as_ref() {
                return Ok(*matrix3);
            }
        }

        Err(ValueError::CannotConvert(value.clone(), "Matrix3".into()))
    }
}

impl TryFrom<&Value> for Direction {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Direction(direction) => Ok(*direction),
            _ => Err(ValueError::CannotConvert(value.clone(), "Direction".into())),
        }
    }
}

impl TryFrom<&Value> for Alignment {
    type Error = ValueError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Alignment(alignment) => Ok(*alignment),
            _ => Err(ValueError::CannotConvert(value.clone(), "Alignment".into())),
        }
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::Quantity((f as Scalar).into())
    }
}

impl From<Scalar> for Value {
    fn from(scalar: Scalar) -> Self {
        Value::Quantity(scalar.into())
    }
}

impl From<Size2> for Value {
    fn from(value: Size2) -> Self {
        Value::Tuple(Box::new(value.into()))
    }
}

impl From<Quantity> for Value {
    fn from(qty: Quantity) -> Self {
        Value::Quantity(qty)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl FromIterator<Value> for Value {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self::Array(iter.into_iter().collect())
    }
}

impl From<Model> for Value {
    fn from(model: Model) -> Self {
        Self::Model(model)
    }
}

impl From<Direction> for Value {
    fn from(direction: Direction) -> Self {
        Value::Direction(direction)
    }
}

impl From<Alignment> for Value {
    fn from(alignment: Alignment) -> Self {
        Value::Alignment(alignment)
    }
}

impl AttributesAccess for Value {
    fn get_attributes_by_id(&self, id: &Identifier) -> Vec<crate::model::Attribute> {
        match self {
            Value::Model(model) => model.get_attributes_by_id(id),
            _ => Vec::default(),
        }
    }
}

#[cfg(test)]
fn integer(value: i64) -> Value {
    Value::Integer(value)
}

#[cfg(test)]
fn scalar(value: f64) -> Value {
    Value::Quantity(Quantity::new(value, QuantityType::Scalar))
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

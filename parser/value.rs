pub type Number = crate::literal::NumberLiteral;

use std::collections::HashMap;

use crate::parser::{Pair, Parse, ParseError, Rule};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug)]
pub enum ValueError {
    InvalidOperation(char),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    // Number value
    Number(Number),

    // Boolean value
    Bool(bool),

    // String value
    String(String),

    // Color value
    Color(Color),

    List(Vec<Value>),

    Array(Vec<Value>),

    NamedTuple(HashMap<String, Value>),

    UnnamedTuple(Vec<Value>),
}

/// Rules for operator +
impl std::ops::Add for Value {
    type Output = Result<Value, ValueError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => (lhs + rhs)
                .map(Value::Number)
                .ok_or(ValueError::InvalidOperation('+')),
            _ => Err(ValueError::InvalidOperation('+')),
        }
    }
}

/// Rules for operator -
impl std::ops::Sub for Value {
    type Output = Result<Value, ValueError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => (lhs - rhs)
                .map(Value::Number)
                .ok_or(ValueError::InvalidOperation('-')),
            _ => Err(ValueError::InvalidOperation('-')),
        }
    }
}

/// Rules for operator *
impl std::ops::Mul for Value {
    type Output = Result<Value, ValueError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => (lhs * rhs)
                .map(Value::Number)
                .ok_or(ValueError::InvalidOperation('*')),
            _ => Err(ValueError::InvalidOperation('*')),
        }
    }
}

/// Rules for operator /
impl std::ops::Div for Value {
    type Output = Result<Value, ValueError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(lhs), Value::Number(rhs)) => (lhs / rhs)
                .map(Value::Number)
                .ok_or(ValueError::InvalidOperation('/')),
            _ => Err(ValueError::InvalidOperation('/')),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Color(c) => write!(f, "rgba({}, {}, {}, {})", c.r, c.g, c.b, c.a),
            Value::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Array(a) => {
                write!(f, "[")?;
                for (i, v) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::NamedTuple(t) => {
                write!(f, "(")?;
                for (i, (name, v)) in t.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", name, v)?;
                }
                write!(f, ")")
            }
            Value::UnnamedTuple(t) => {
                write!(f, "(")?;
                for (i, v) in t.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, ")")
            }
        }
    }
}

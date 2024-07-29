pub type Number = crate::literal::NumberLiteral;

use std::collections::HashMap;

use crate::{
    identifier::Identifier,
    langtype::{Ty, Type},
    syntax_tree::SyntaxNode,
};

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
    ArrayLengthMismatchForOperator {
        operator: char,
        lhs: usize,
        rhs: usize,
    },
    TupleLengthMismatchForOperator {
        operator: char,
        lhs: usize,
        rhs: usize,
    },
}

pub type Scalar = f64;

pub type Vec2 = euclid::Vector2D<Scalar, ()>;
pub type Vec3 = euclid::Vector3D<Scalar, ()>;

#[derive(Clone, Debug, PartialEq)]
pub struct List(pub Vec<Value>, pub Option<Type>);

impl List {
    pub fn new() -> Self {
        Self(Vec::new(), None)
    }

    pub fn push(&mut self, value: Value) {
        self.0.push(value);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.0.get(index)
    }

    pub fn extend(&mut self, other: Self) {
        self.0.extend(other.0);
    }

    pub fn contains(&self, value: &Value) -> bool {
        self.0.contains(value)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Value) -> bool,
    {
        self.0.retain(f);
    }

    pub fn iter(&self) -> std::slice::Iter<Value> {
        self.0.iter()
    }
}

impl Ty for List {
    fn ty(&self) -> Type {
        if let Some(t) = &self.1 {
            Type::List(Some(Box::new(t.clone())))
        } else {
            Type::List(None)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Array(pub Vec<Value>, pub Type);

impl Array {
    pub fn new(ty: Type) -> Self {
        Self(Vec::new(), ty)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<Value> {
        self.0.iter()
    }
}

impl Ty for Array {
    fn ty(&self) -> Type {
        Type::Array(Box::new(self.1.clone()), self.0.len())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamedTuple(pub HashMap<Identifier, Value>);

impl NamedTuple {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Identifier, &Value)> {
        self.0.iter()
    }
}

impl Ty for NamedTuple {
    fn ty(&self) -> Type {
        Type::NamedTuple(self.0.iter().map(|(k, v)| (k.clone(), v.ty())).collect())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnnamedTuple(pub Vec<Value>);

impl UnnamedTuple {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> std::slice::Iter<Value> {
        self.0.iter()
    }
}

impl Ty for UnnamedTuple {
    fn ty(&self) -> Type {
        Type::UnnamedTuple(self.0.iter().map(Value::ty).collect())
    }
}

impl std::ops::Add for UnnamedTuple {
    type Output = Result<UnnamedTuple, ValueError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.len() != rhs.len() {
            return Err(ValueError::TupleLengthMismatchForOperator {
                operator: '+',
                lhs: self.len(),
                rhs: rhs.len(),
            });
        }
        let mut result = Vec::new();
        for (l, r) in self.0.into_iter().zip(rhs.0.into_iter()) {
            result.push((l + r)?);
        }
        Ok(UnnamedTuple(result))
    }
}

impl std::ops::Sub for UnnamedTuple {
    type Output = Result<UnnamedTuple, ValueError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.len() != rhs.len() {
            return Err(ValueError::TupleLengthMismatchForOperator {
                operator: '-',
                lhs: self.len(),
                rhs: rhs.len(),
            });
        }
        let mut result = Vec::new();
        for (l, r) in self.0.into_iter().zip(rhs.0.into_iter()) {
            result.push((l - r)?);
        }
        Ok(UnnamedTuple(result))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    // An integer value
    Integer(i64),

    // A scalar value
    Scalar(Scalar),

    // Length in mm
    Length(Scalar),

    // A 2D vector with length
    Vec2(Vec2),

    // A 3D vector with length
    Vec3(Vec3),

    // An angle in radians
    Angle(Scalar),

    // Boolean value
    Bool(bool),

    // String value
    String(String),

    // Color value
    Color(Color),

    List(List),

    Array(Array),

    NamedTuple(NamedTuple),

    UnnamedTuple(UnnamedTuple),
    // TODO: Add syntax node as value
    //Node(SyntaxNode)
}

impl Ty for Value {
    fn ty(&self) -> Type {
        match self {
            Value::Integer(_) => Type::Integer,
            Value::Scalar(_) => Type::Scalar,
            Value::Length(_) => Type::Length,
            Value::Vec2(_) => Type::Vec2,
            Value::Vec3(_) => Type::Vec3,
            Value::Angle(_) => Type::Angle,
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Color(_) => Type::Color,
            Value::List(list) => list.ty(),
            Value::Array(array) => array.ty(),
            Value::NamedTuple(named_tuple) => named_tuple.ty(),
            Value::UnnamedTuple(unnamed_tuple) => unnamed_tuple.ty(),
        }
    }
}

/// Rules for operator +
impl std::ops::Add for Value {
    type Output = Result<Value, ValueError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
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
            (Value::List(mut lhs), Value::List(rhs)) => {
                lhs.extend(rhs);
                Ok(Value::List(lhs))
            }
            // Add values of two tuples of the same length
            (Value::UnnamedTuple(lhs), Value::UnnamedTuple(rhs)) => {
                Ok(Value::UnnamedTuple((lhs + rhs)?))
            }
            _ => Err(ValueError::InvalidOperation('+')),
        }
    }
}

/// Rules for operator -
impl std::ops::Sub for Value {
    type Output = Result<Value, ValueError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
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
            _ => Err(ValueError::InvalidOperation('-')),
        }
    }
}

/// Rules for operator *
impl std::ops::Mul for Value {
    type Output = Result<Value, ValueError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
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

            _ => Err(ValueError::InvalidOperation('*')),
        }
    }
}

/// Rules for operator /
impl std::ops::Div for Value {
    type Output = Result<Value, ValueError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Divide two scalars
            (Value::Scalar(lhs), Value::Scalar(rhs))
            | (Value::Length(lhs), Value::Length(rhs))
            | (Value::Angle(lhs), Value::Angle(rhs)) => Ok(Value::Scalar(lhs / rhs)),
            (Value::Length(lhs), Value::Scalar(rhs)) => Ok(Value::Length(lhs / rhs)),
            (Value::Angle(lhs), Value::Scalar(rhs)) => Ok(Value::Angle(lhs / rhs)),
            _ => Err(ValueError::InvalidOperation('/')),
        }
    }
}

/// Rules for operators <, <=, ==, >=, >
impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Integer(lhs), Value::Integer(rhs)) => lhs.partial_cmp(rhs),
            (Value::Scalar(lhs), Value::Scalar(rhs)) => lhs.partial_cmp(rhs),
            (Value::Length(lhs), Value::Length(rhs)) => lhs.partial_cmp(rhs),
            (Value::Angle(lhs), Value::Angle(rhs)) => lhs.partial_cmp(rhs),
            _ => None,
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

pub type Number = crate::literal::NumberLiteral;

use std::collections::{BTreeMap, HashMap};
use thiserror::Error;

use crate::{
    color::Color,
    identifier::Identifier,
    langtype::{MapKeyType, NamedTupleType, Ty, Type, TypeList, UnnamedTupleType},
    syntax_tree::{qualified_name, SyntaxNode},
    units::Unit,
};

#[derive(Debug, Error)]
pub enum ValueError {
    #[error("Invalid operator: {0}")]
    InvalidOperator(char),
    #[error("Tuple length mismatch for operator {operator}: lhs={lhs}, rhs={rhs}")]
    TupleLengthMismatchForOperator {
        operator: char,
        lhs: usize,
        rhs: usize,
    },
    #[error("Type cannot be a key in a map: {0}")]
    InvalidMapKeyType(Type),
    #[error("Cannot convert value into scalar: {0}")]
    CannotConvertToScalar(Value),
}

pub type Scalar = f64;

pub type Vec2 = euclid::Vector2D<Scalar, ()>;
pub type Vec3 = euclid::Vector3D<Scalar, ()>;

#[derive(Clone, Debug, PartialEq)]
pub struct List(pub ValueList, pub Type);

impl List {
    pub fn new(ty: Type) -> Self {
        Self(ValueList::new(), ty)
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

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

impl Ty for List {
    fn ty(&self) -> Type {
        self.1.clone()
    }
}

/// A value type that can be used as a key in a map
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum MapKeyValue {
    Integer(i64),
    Bool(bool),
    String(String),
}

impl MapKeyValue {
    pub fn ty(&self) -> MapKeyType {
        match self {
            MapKeyValue::Integer(_) => MapKeyType::Integer,
            MapKeyValue::Bool(_) => MapKeyType::Bool,
            MapKeyValue::String(_) => MapKeyType::String,
        }
    }
}

impl TryFrom<Value> for MapKeyValue {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Integer(n) => Ok(MapKeyValue::Integer(n)),
            Value::Bool(b) => Ok(MapKeyValue::Bool(b)),
            Value::String(s) => Ok(MapKeyValue::String(s)),
            value => Err(ValueError::InvalidMapKeyType(value.ty())),
        }
    }
}

impl std::fmt::Display for MapKeyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapKeyValue::Integer(n) => write!(f, "{}", n),
            MapKeyValue::Bool(b) => write!(f, "{}", b),
            MapKeyValue::String(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Map(pub HashMap<MapKeyValue, Value>, pub MapKeyType, Type);

impl Map {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Ty for Map {
    fn ty(&self) -> Type {
        self.2.clone()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, (k, v)) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{} => {}", k, v)?;
        }
        write!(f, "]")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamedTuple(pub BTreeMap<Identifier, Value>);

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

impl std::fmt::Display for NamedTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, (name, v)) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{} = {}", name, v)?;
        }
        write!(f, ")")
    }
}

impl Ty for NamedTuple {
    fn ty(&self) -> Type {
        Type::NamedTuple(NamedTupleType(
            self.0
                .iter()
                .map(|(name, v)| (name.clone(), v.ty().clone()))
                .collect(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnnamedTuple(pub ValueList);

impl UnnamedTuple {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> std::slice::Iter<Value> {
        self.0.iter()
    }

    pub fn binary_op(
        self,
        rhs: Self,
        op: char,
        f: impl Fn(Value, Value) -> Result<Value, ValueError>,
    ) -> Result<Self, ValueError> {
        if self.len() != rhs.len() {
            return Err(ValueError::TupleLengthMismatchForOperator {
                operator: op,
                lhs: self.len(),
                rhs: rhs.len(),
            });
        }
        let mut result = ValueList::new();
        for (l, r) in self.0.iter().zip(rhs.0.iter()) {
            let add_result = f(l.clone(), r.clone())?;
            result.push(add_result);
        }
        Ok(UnnamedTuple(result))
    }
}

impl std::fmt::Display for UnnamedTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, v) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, ")")
    }
}

impl Ty for UnnamedTuple {
    fn ty(&self) -> Type {
        Type::UnnamedTuple(UnnamedTupleType(
            self.0.iter().map(|v| v.ty().clone()).collect(),
        ))
    }
}

impl std::ops::Add for UnnamedTuple {
    type Output = Result<UnnamedTuple, ValueError>;

    fn add(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, '+', |lhs, rhs| lhs + rhs)
    }
}

impl std::ops::Sub for UnnamedTuple {
    type Output = Result<UnnamedTuple, ValueError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, '-', |lhs, rhs| lhs - rhs)
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

    Map(Map),

    NamedTuple(NamedTuple),

    UnnamedTuple(UnnamedTuple),

    // TODO: Add syntax node as value
    Node(SyntaxNode),
}

impl Value {
    pub fn less_than(&self, rhs: &Self) -> Result<bool, ValueError> {
        match (self, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(lhs < rhs),
            (Value::Scalar(lhs), Value::Scalar(rhs)) => Ok(lhs < rhs),
            (Value::Length(lhs), Value::Length(rhs)) => Ok(lhs < rhs),
            (Value::Vec2(lhs), Value::Vec2(rhs)) => Ok(lhs.length() < rhs.length()),
            (Value::Vec3(lhs), Value::Vec3(rhs)) => Ok(lhs.length() < rhs.length()),
            (Value::Angle(lhs), Value::Angle(rhs)) => Ok(lhs < rhs),
            _ => Err(ValueError::InvalidOperator('<')),
        }
    }

    pub fn greater_than(&self, rhs: &Self) -> Result<bool, ValueError> {
        match (self, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(lhs > rhs),
            (Value::Scalar(lhs), Value::Scalar(rhs)) => Ok(lhs > rhs),
            (Value::Length(lhs), Value::Length(rhs)) => Ok(lhs > rhs),
            (Value::Vec2(lhs), Value::Vec2(rhs)) => Ok(lhs.length() > rhs.length()),
            (Value::Vec3(lhs), Value::Vec3(rhs)) => Ok(lhs.length() > rhs.length()),
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

    pub fn into_scalar(&self) -> Result<Scalar, ValueError> {
        match self {
            Value::Scalar(s) | Value::Length(s) | Value::Angle(s) => Ok(*s),
            value => Err(ValueError::CannotConvertToScalar(value.clone())),
        }
    }

    /// Add a unit to a scalar value
    pub fn add_unit_to_scalar_types(&mut self, unit: Unit) {
        match (&self, unit.ty()) {
            (Value::Integer(i), Type::Length) => {
                *self = Value::Length(unit.normalize(*i as Scalar))
            }
            (Value::Integer(i), Type::Angle) => *self = Value::Angle(unit.normalize(*i as Scalar)),
            (Value::Scalar(s), Type::Length) => *self = Value::Length(unit.normalize(*s)),
            (Value::Scalar(s), Type::Angle) => *self = Value::Angle(unit.normalize(*s)),
            //(Value::List(list), _) => list.add_unit_to_scalar_types(unit),
            _ => {}
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
            Value::Angle(_) => Type::Angle,
            Value::Bool(_) => Type::Bool,
            Value::String(_) => Type::String,
            Value::Color(_) => Type::Color,
            Value::List(list) => list.ty(),
            Value::Map(map) => map.ty(),
            Value::NamedTuple(named_tuple) => named_tuple.ty(),
            Value::UnnamedTuple(unnamed_tuple) => unnamed_tuple.ty(),
            Value::Node(node) => {
                if let Some(qualified_name) = crate::syntax_tree::qualified_name(node.clone()) {
                    Type::Custom(qualified_name)
                } else {
                    // TODO Trait Ty should return a Result<Type, TypeError>
                    panic!("Cannot get type of node: {:?}", node);
                }
            }
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
            (Value::List(mut lhs), Value::List(rhs)) => {
                lhs.extend(rhs);
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
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Color(c) => write!(f, "{}", c),
            Value::List(l) => write!(f, "{}", l),
            Value::Map(m) => write!(f, "{}", m),
            Value::NamedTuple(t) => write!(f, "{}", t),
            Value::UnnamedTuple(t) => write!(f, "{}", t),
            Value::Node(node) => {
                write!(
                    f,
                    "{}",
                    qualified_name(node.clone()).unwrap_or("<unknown>".into())
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValueList(Vec<Value>);

impl ValueList {
    pub fn new() -> Self {
        Self(Vec::new())
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

    pub fn iter(&self) -> std::slice::Iter<Value> {
        self.0.iter()
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

    pub fn add_unit_to_scalar_types(&mut self, unit: Unit) {
        for value in self.0.iter_mut() {
            value.add_unit_to_scalar_types(unit);
        }
    }

    pub fn types(&self) -> TypeList {
        TypeList::from_types(
            self.0
                .iter()
                .map(|v| v.ty())
                .collect::<Vec<Type>>()
                .into_iter()
                .collect(),
        )
    }
}

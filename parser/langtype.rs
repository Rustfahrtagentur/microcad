use std::collections::HashMap;
use thiserror::Error;

use crate::{
    identifier::{Identifier, QualifiedName},
    parser::{Pair, Parse, ParseError, Rule},
    units,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ListType(Box<Type>);

impl ListType {
    fn from_type(t: Type) -> Self {
        Self(Box::new(t))
    }
}

impl Ty for ListType {
    fn ty(&self) -> Type {
        self.0.as_ref().clone()
    }
}

impl Parse for ListType {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut inner = pair.into_inner();

        let pair = inner.next().unwrap();
        match pair.as_rule() {
            Rule::r#type => Ok(Self::from_type(Type::parse(pair)?)),
            _ => unreachable!("Expected type, found {:?}", pair.as_rule()),
        }
    }
}

impl std::fmt::Display for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum MapKeyType {
    #[default]
    Integer,
    Bool,
    String,
}

impl TryFrom<Type> for MapKeyType {
    type Error = TypeError;

    fn try_from(t: Type) -> Result<Self, Self::Error> {
        match t {
            Type::Integer => Ok(Self::Integer),
            Type::Bool => Ok(Self::Bool),
            Type::String => Ok(Self::String),
            _ => Err(TypeError::InvalidMapKeyType(t.to_string())),
        }
    }
}

impl std::fmt::Display for MapKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "int"),
            Self::Bool => write!(f, "bool"),
            Self::String => write!(f, "string"),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapType(MapKeyType, Box<Type>);

impl MapType {
    fn from_types(key: MapKeyType, value: Type) -> Self {
        Self(key, Box::new(value))
    }
}

impl Parse for MapType {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut inner = pair.into_inner();

        let key = inner.next().unwrap();
        let value = inner.next().unwrap();

        Ok(Self::from_types(
            Type::parse(key)?.try_into()?,
            Type::parse(value)?,
        ))
    }
}

impl std::fmt::Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} => {}]", self.0, self.1)
    }
}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Invalid map key type: {0}")]
    InvalidMapKeyType(String),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Type {
    /// Correspond to an uninitialized type, or an error
    #[default]
    Invalid,

    // A 64-bit integer number
    Integer,

    /// A 64-bit floating-point number
    Scalar,

    /// A string
    String,

    /// An RGBA color
    Color,

    /// A physical length, e.g. 4.0mm
    Length,

    /// An angle, e.g. 90°
    Angle,

    /// A two-dimensional vector
    Vec2,

    /// A three-dimensional vector
    Vec3,

    /// A boolean
    Bool,

    List(ListType),

    Map(MapType),

    UnnamedTuple(Vec<Type>),

    NamedTuple(HashMap<Identifier, Type>),

    /// A node in the syntax tree
    Custom(QualifiedName),
}

impl Type {
    pub fn default_unit(&self) -> units::Unit {
        match self {
            Self::Length => units::Unit::Mm,
            Self::Angle => units::Unit::Rad,
            Self::List(t) => t.ty().default_unit(),
            _ => units::Unit::None,
        }
    }
}

impl Parse for Type {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let inner = pair.into_inner().next().unwrap();

        match inner.as_rule() {
            Rule::list_type => Ok(Self::List(ListType::parse(inner)?)),
            Rule::map_type => Ok(Self::Map(MapType::parse(inner)?)),
            Rule::qualified_name => match inner.as_str() {
                "int" => Ok(Self::Integer),
                "scalar" => Ok(Self::Scalar),
                "string" => Ok(Self::String),
                "color" => Ok(Self::Color),
                "length" => Ok(Self::Length),
                "angle" => Ok(Self::Angle),
                "vec2" => Ok(Self::Vec2),
                "vec3" => Ok(Self::Vec3),
                "bool" => Ok(Self::Bool),
                _ => Ok(Self::Custom(QualifiedName::parse(inner)?)),
            },
            _ => unreachable!("Expected type, found {:?}", inner.as_rule()),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "int"),
            Self::Invalid => write!(f, "invalid"),
            Self::Scalar => write!(f, "scalar"),
            Self::String => write!(f, "string"),
            Self::Color => write!(f, "color"),
            Self::Length => write!(f, "length"),
            Self::Angle => write!(f, "angle"),
            Self::Vec2 => write!(f, "vec2"),
            Self::Vec3 => write!(f, "vec3"),
            Self::Bool => write!(f, "bool"),
            Self::List(t) => write!(f, "{}", t),
            Self::Map(t) => write!(f, "{}", t),
            Self::UnnamedTuple(t) => {
                write!(f, "(")?;
                for (i, t) in t.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Self::NamedTuple(t) => {
                write!(f, "(")?;
                for (i, (name, t)) in t.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", name, t)?;
                }
                write!(f, ")")
            }
            Self::Custom(qn) => write!(f, "{}", qn),
        }
    }
}

/// Trait for structs and expressions that have a type
pub trait Ty {
    fn ty(&self) -> Type;
}

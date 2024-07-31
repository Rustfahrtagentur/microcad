use std::collections::BTreeMap;
use thiserror::Error;

use crate::{
    identifier::{Identifier, QualifiedName},
    parser::{Pair, Parse, ParseError, Rule},
    units,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ListType(Box<Type>);

impl ListType {
    pub fn from_type(t: Type) -> Self {
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnnamedTupleType(pub Vec<Type>);

impl Parse for UnnamedTupleType {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let inner = pair.into_inner();

        let mut types = Vec::new();
        for pair in inner {
            types.push(Type::parse(pair)?);
        }

        Ok(Self(types))
    }
}

impl std::fmt::Display for UnnamedTupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, t) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", t)?;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NamedTupleType(pub BTreeMap<Identifier, Type>);

impl Parse for NamedTupleType {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut types = BTreeMap::new();
        for pair in pair.into_inner() {
            let mut inner = pair.into_inner();
            let name = Identifier::parse(inner.next().unwrap())?;
            let ty = Type::parse(inner.next().unwrap())?;
            if types.contains_key(&name) {
                return Err(TypeError::DuplicatedMapField(name.clone()).into());
            }
            types.insert(name, ty);
        }

        Ok(Self(types))
    }
}

impl std::fmt::Display for NamedTupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, (name, ty)) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", name, ty)?;
        }
        write!(f, ")")
    }
}

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Invalid map key type: {0}")]
    InvalidMapKeyType(String),

    #[error("Duplicated field name in map: {0}")]
    DuplicatedMapField(Identifier),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
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

    /// A two-dimensional vector, maps from named tuple ((x,y): length)
    Vec2,

    /// A three-dimensional vector, maps from named tuple ((x,y,z): length)
    Vec3,

    /// A boolean: true, false
    Bool,

    /// A list of elements of the same type: [scalar]
    List(ListType),

    /// A map of elements: [string => scalar]
    Map(MapType),

    /// An unnamed tuple of elements: (scalar, string)
    UnnamedTuple(UnnamedTupleType),

    /// A named tuple of elements: (x: scalar, y: string)
    NamedTuple(NamedTupleType),

    /// A custom type or a module node in the syntax tree
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
            Rule::unnamed_tuple_type => Ok(Self::UnnamedTuple(UnnamedTupleType::parse(inner)?)),
            Rule::named_tuple_type => Ok(Self::NamedTuple(NamedTupleType::parse(inner)?)),
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
            Self::UnnamedTuple(t) => write!(f, "{}", t),
            Self::NamedTuple(t) => write!(f, "{}", t),
            Self::Custom(qn) => write!(f, "{}", qn),
        }
    }
}

/// Trait for structs and expressions that have a type
pub trait Ty {
    fn ty(&self) -> Type;
}

pub struct TypeList(Vec<Type>);

impl TypeList {
    pub fn from_types(types: Vec<Type>) -> Self {
        Self(types)
    }

    pub fn common_type(&self) -> Option<Type> {
        let mut common_type = None;
        for ty in &self.0 {
            match common_type {
                None => common_type = Some(ty.clone()),
                Some(ref t) if t == ty => {}
                _ => return None,
            }
        }
        common_type
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lang_type::Type,
        parser::{Parser, Rule},
    };

    #[test]
    fn builtin_type() {
        let ty = Parser::parse_rule_or_panic::<Type>(Rule::r#type, "int");
        assert_eq!(ty.to_string(), "int");
        assert_eq!(ty, Type::Integer);
    }

    #[test]
    fn list_type() {
        use crate::lang_type::ListType;
        let ty = Parser::parse_rule_or_panic::<Type>(Rule::r#type, "[int]");
        assert_eq!(ty.to_string(), "[int]");
        assert_eq!(ty, Type::List(ListType::from_type(Type::Integer)));
    }

    #[test]
    fn map_type() {
        use crate::lang_type::MapType;
        let ty = Parser::parse_rule_or_panic::<Type>(Rule::r#type, "[int => string]");
        assert_eq!(ty.to_string(), "[int => string]");
        assert_eq!(
            ty,
            Type::Map(MapType::from_types(
                crate::lang_type::MapKeyType::Integer,
                Type::String
            ))
        );
    }

    #[test]
    fn unnamed_tuple_type() {
        use crate::lang_type::UnnamedTupleType;
        let ty = Parser::parse_rule_or_panic::<Type>(Rule::r#type, "(int, string)");
        assert_eq!(ty.to_string(), "(int, string)");
        assert_eq!(
            ty,
            Type::UnnamedTuple(UnnamedTupleType(vec![Type::Integer, Type::String]))
        );
    }

    #[test]
    fn named_tuple_type() {
        use crate::identifier::Identifier;
        use crate::lang_type::NamedTupleType;

        let ty = Parser::parse_rule_or_panic::<Type>(Rule::r#type, "(x: int, y: string)");
        assert_eq!(ty.to_string(), "(x: int, y: string)");
        assert_eq!(
            ty,
            Type::NamedTuple(NamedTupleType(
                vec![
                    (Identifier::from("x"), Type::Integer),
                    (Identifier::from("y"), Type::String)
                ]
                .into_iter()
                .collect()
            ))
        );
    }
}

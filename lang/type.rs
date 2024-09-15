// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD Basic Types

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// µCAD Basic Types
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// A 64-bit integer number
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
    /// A physical weight, e.g. 4.0kg
    Weight,
    /// A two-dimensional vector, maps from named tuple (x: length, y: length)    
    Vec2,
    /// A three-dimensional vector, maps from named tuple (x: length, y: length, z: length)
    Vec3,
    /// A three-dimensional vector, maps from named tuple (x: length, y: length, z: length, w: length)
    Vec4,
    /// A boolean: true, false
    Bool,
    /// A list of elements of the same type: `[scalar]`
    List(ListType),
    /// A map of elements: `[string => scalar]`
    Map(MapType),
    /// An unnamed tuple of elements: `(scalar, string)`
    UnnamedTuple(UnnamedTupleType),
    /// A named tuple of elements: `(x: scalar, y: string)`
    NamedTuple(NamedTupleType),
    /// A custom type or a module node in the syntax tree
    Custom(QualifiedName),
    /// Node
    Node,
}

impl Type {
    /// Return default unit if primitive type or list of primitive types)
    pub fn default_unit(&self) -> Unit {
        match self {
            Self::Length => Unit::Mm,
            Self::Angle => Unit::Rad,
            Self::List(t) => t.ty().default_unit(),
            _ => Unit::None,
        }
    }

    /// Check if the type is a named tuple
    pub fn is_named_tuple(&self) -> bool {
        matches!(self, Self::NamedTuple(_))
    }

    /// Check if the type is a list of the given type `ty`
    pub fn is_list_of(&self, ty: &Type) -> bool {
        match self {
            Self::List(list_type) => &list_type.ty() == ty,
            _ => false,
        }
    }
}

/// Type within source code
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation(pub Refer<Type>);

impl SrcReferrer for TypeAnnotation {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Ty for TypeAnnotation {
    fn ty(&self) -> Type {
        self.0.value.clone()
    }
}

impl From<Type> for TypeAnnotation {
    fn from(value: Type) -> Self {
        TypeAnnotation(Refer::none(value))
    }
}

impl Parse for TypeAnnotation {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::r#type);
        let inner = pair.inner().next().unwrap();

        let s = match inner.as_rule() {
            Rule::list_type => Self(Refer::new(Type::List(ListType::parse(inner)?), pair.into())),
            Rule::map_type => Self(Refer::new(Type::Map(MapType::parse(inner)?), pair.into())),
            Rule::unnamed_tuple_type => Self(Refer::new(
                Type::UnnamedTuple(UnnamedTupleType::parse(inner)?),
                pair.into(),
            )),
            Rule::named_tuple_type => Self(Refer::new(
                Type::NamedTuple(NamedTupleType::parse(inner)?),
                pair.into(),
            )),
            Rule::qualified_name => match inner.as_str() {
                "int" => Self(Refer::new(Type::Integer, pair.into())),
                "scalar" => Self(Refer::new(Type::Scalar, pair.into())),
                "string" => Self(Refer::new(Type::String, pair.into())),
                "color" => Self(Refer::new(Type::Color, pair.into())),
                "length" => Self(Refer::new(Type::Length, pair.into())),
                "angle" => Self(Refer::new(Type::Angle, pair.into())),
                "vec2" => Self(Refer::new(Type::Vec2, pair.into())),
                "vec3" => Self(Refer::new(Type::Vec3, pair.into())),
                "bool" => Self(Refer::new(Type::Bool, pair.into())),
                _ => Self(Refer::new(
                    Type::Custom(QualifiedName::parse(inner)?),
                    pair.into(),
                )),
            },
            _ => unreachable!("Expected type, found {:?}", inner.as_rule()),
        };

        Ok(s)
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "int"),
            Self::Scalar => write!(f, "scalar"),
            Self::String => write!(f, "string"),
            Self::Color => write!(f, "color"),
            Self::Length => write!(f, "length"),
            Self::Angle => write!(f, "angle"),
            Self::Weight => write!(f, "weight"),
            Self::Vec2 => write!(f, "vec2"),
            Self::Vec3 => write!(f, "vec3"),
            Self::Vec4 => write!(f, "vec4"),
            Self::Bool => write!(f, "bool"),
            Self::List(t) => write!(f, "{}", t),
            Self::Map(t) => write!(f, "{}", t),
            Self::UnnamedTuple(t) => write!(f, "{}", t),
            Self::NamedTuple(t) => write!(f, "{}", t),
            Self::Custom(qn) => write!(f, "{}", qn),
            Self::Node => write!(f, "{{}}"),
        }
    }
}

#[test]
fn builtin_type() {
    let ty = Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "int", 0).unwrap();
    assert_eq!(ty.0.to_string(), "int");
    assert_eq!(ty.0.value, Type::Integer);
}

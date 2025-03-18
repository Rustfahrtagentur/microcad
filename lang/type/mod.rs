// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Basic Types

mod type_annotation;

pub use type_annotation::*;

use crate::parse::*;

/// µcad Basic Types
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Invalid type (used for error handling)
    Invalid,
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
    /// A physical area, e.g. 4.0mm²
    Area,
    /// A physical volume, e.g. 4.0mm³
    Volume,
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
            Self::Length => Unit::Millimeter,
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

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, "<invalid>"),
            Self::Integer => write!(f, "Int"),
            Self::Scalar => write!(f, "Scalar"),
            Self::String => write!(f, "String"),
            Self::Color => write!(f, "Color"),
            Self::Length => write!(f, "Length"),
            Self::Area => write!(f, "Area"),
            Self::Volume => write!(f, "Volume"),
            Self::Angle => write!(f, "Angle"),
            Self::Weight => write!(f, "Weight"),
            Self::Vec2 => write!(f, "Vec2"),
            Self::Vec3 => write!(f, "Vec3"),
            Self::Vec4 => write!(f, "Vec4"),
            Self::Bool => write!(f, "Bool"),
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
    use crate::parser::*;

    let ty = Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "Int", 0).expect("test error");
    assert_eq!(ty.0.to_string(), "Int");
    assert_eq!(ty.0.value, Type::Integer);
}

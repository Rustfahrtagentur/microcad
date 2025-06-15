// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Type

use crate::{syntax::*, ty::*};

/// µcad Basic Types
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Invalid type (used for error handling)
    Invalid,
    /// A 64-bit integer number
    Integer,
    /// A 64-bit floating-point number
    Quantity(QuantityType),
    /// A string
    String,
    /// An RGBA color
    Color,
    /// A boolean: true, false
    Bool,
    /// A list of elements of the same type: `[Scalar]`
    List(ListType),
    /// An unnamed tuple of elements: `(Scalar, String)`
    Tuple(TupleType),
    /// A named tuple of elements: `(x: Scalar, y: String)`
    NamedTuple(NamedTupleType),
    /// Matrix type
    Matrix(MatrixType),
    /// Nodes.
    Nodes,
    /// A custom type or a part node in the syntax tree
    Custom(QualifiedName),
}

impl Type {
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

impl From<QuantityType> for Type {
    fn from(value: QuantityType) -> Self {
        Type::Quantity(value)
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, "<invalid>"),
            Self::Integer => write!(f, "Integer"),
            Self::Quantity(quantity) => write!(f, "{quantity}"),
            Self::String => write!(f, "String"),
            Self::Color => write!(f, "Color"),
            Self::Bool => write!(f, "Bool"),
            Self::List(t) => write!(f, "{t}"),
            Self::Tuple(t) => write!(f, "{t}"),
            Self::NamedTuple(t) => write!(f, "{t}"),
            Self::Matrix(t) => write!(f, "{t}"),
            Self::Nodes => write!(f, "Nodes"),
            Self::Custom(n) => write!(f, "Custom({n})"),
        }
    }
}

pub enum BuiltinTypeWrapper {
    Integer,

    Scalar,

    Angle,

    String,
}

impl From<BuiltinTypeWrapper> for Type {
    fn from(value: BuiltinTypeWrapper) -> Self {
        match value {
            BuiltinTypeWrapper::Integer => Type::Integer,
            BuiltinTypeWrapper::Scalar => Type::Quantity(QuantityType::Scalar),
            BuiltinTypeWrapper::Angle => Type::Quantity(QuantityType::Angle),
            BuiltinTypeWrapper::String => Type::String,
        }
    }
}

#[test]
fn builtin_type() {
    use crate::parser::*;

    let ty = Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "Integer", 0).expect("test error");
    assert_eq!(ty.0.to_string(), "Integer");
    assert_eq!(ty.0.value, Type::Integer);
}

// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Type

use crate::{syntax::*, ty::*};

/// µcad Basic Types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// Invalid type (used for error handling)
    Invalid,
    /// A 64-bit integer number
    Integer,
    /// A 64-bit floating-point number
    Quantity(QuantityType),
    /// A string
    String,
    /// A boolean: true, false
    Bool,
    /// A list of elements of the same type: `[Scalar]`
    List(ListType),
    /// A named tuple of elements: `(x: Scalar, y: String)`
    Tuple(Box<TupleType>),
    /// Matrix type
    Matrix(MatrixType),
    /// Nodes.
    Nodes,
    /// A custom type or a part node in the syntax tree
    Custom(QualifiedName),
}

impl Type {
    /// Shortcut to create a scalar type.
    pub fn scalar() -> Self {
        Self::Quantity(QuantityType::Scalar)
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
            Self::Bool => write!(f, "Bool"),
            Self::List(t) => write!(f, "{t}"),
            Self::Tuple(t) => write!(f, "{t}"),
            Self::Matrix(t) => write!(f, "{t}"),
            Self::Nodes => write!(f, "Nodes"),
            Self::Custom(n) => write!(f, "Custom({n})"),
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

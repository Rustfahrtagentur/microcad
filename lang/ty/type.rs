// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Type

use crate::{syntax::*, ty::*};

/// µcad Basic Types
#[derive(Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Type {
    /// Invalid type (used for error handling)
    Invalid,
    /// A 64-bit integer number: `Interger: 10`.
    Integer,
    /// A quantity type: `Scalar: 1.0`, `Length: 1.0mm`
    Quantity(QuantityType),
    /// A string.
    String,
    /// A boolean: `true`, `false`.
    Bool,
    /// An array of elements of the same type: `[Scalar]`.
    Array(Box<Type>),
    /// A named tuple of elements: `(x: Scalar, y: String)`.
    Tuple(Box<TupleType>),
    /// Matrix type: `Matrix3x3`.
    Matrix(MatrixType),
    /// Models.
    Models,
    /// A custom type in the syntax tree
    Custom(QualifiedName),
}

impl Type {
    /// Shortcut to create a scalar type.
    pub fn scalar() -> Self {
        Self::Quantity(QuantityType::Scalar)
    }

    /// Shortcut to create a length type.
    pub fn length() -> Self {
        Self::Quantity(QuantityType::Length)
    }

    /// Check if the type is an array of the given type `ty`
    pub fn is_array_of(&self, ty: &Type) -> bool {
        match self {
            Self::Array(array_type) => array_type.as_ref() == ty,
            _ => false,
        }
    }
}

impl std::ops::Mul for Type {
    type Output = Type;

    fn mul(self, rhs: Self) -> Self::Output {
        if self == Self::Invalid || rhs == Self::Invalid {
            return Self::Invalid;
        }

        match (self, rhs) {
            (Type::Integer, ty) | (ty, Type::Integer) => ty,
            (Type::Quantity(lhs), Type::Quantity(rhs)) => Type::Quantity(lhs * rhs),
            (ty, Type::Array(array_type)) | (Type::Array(array_type), ty) => *array_type * ty,
            (Type::Tuple(_), _) | (_, Type::Tuple(_)) => todo!(),
            (Type::Matrix(_), _) | (_, Type::Matrix(_)) => todo!(),
            (lhs, rhs) => unimplemented!("Multiplication for {lhs} * {rhs}"),
        }
    }
}

impl std::ops::Div for Type {
    type Output = Type;

    fn div(self, rhs: Self) -> Self::Output {
        if self == Self::Invalid || rhs == Self::Invalid {
            return Self::Invalid;
        }

        match (self, rhs) {
            (ty, Type::Integer) => ty,
            (Type::Quantity(lhs), Type::Quantity(rhs)) => Type::Quantity(lhs / rhs),
            (Type::Array(array_type), ty) => *array_type / ty,
            (Type::Tuple(_), _) => todo!(),
            (Type::Matrix(_), _) | (_, Type::Matrix(_)) => todo!(),
            (lhs, rhs) => unimplemented!("Division for {lhs} * {rhs}"),
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
            Self::Invalid => write!(f, crate::invalid_no_ansi!(TYPE)),
            Self::Integer => write!(f, "Integer"),
            Self::Quantity(quantity) => write!(f, "{quantity}"),
            Self::String => write!(f, "String"),
            Self::Bool => write!(f, "Bool"),
            Self::Array(t) => write!(f, "[{t}]"),
            Self::Tuple(t) => write!(f, "{t}"),
            Self::Matrix(t) => write!(f, "{t}"),
            Self::Models => write!(f, "Models"),
            Self::Custom(n) => write!(f, "Custom({n})"),
        }
    }
}

impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Invalid => write!(f, crate::invalid!(TYPE)),
            Self::Integer => write!(f, "Integer"),
            Self::Quantity(quantity) => write!(f, "{quantity}"),
            Self::String => write!(f, "String"),
            Self::Bool => write!(f, "Bool"),
            Self::Array(t) => write!(f, "[{t}]"),
            Self::Tuple(t) => write!(f, "{t}"),
            Self::Matrix(t) => write!(f, "{t}"),
            Self::Models => write!(f, "Models"),
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

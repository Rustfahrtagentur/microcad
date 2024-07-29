use std::collections::HashMap;

use crate::{
    identifier::{Identifier, QualifiedName},
    units,
};

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

    List(Option<Box<Type>>),

    Array(Box<Type>, usize),

    UnnamedTuple(Vec<Type>),

    NamedTuple(HashMap<Identifier, Type>),

    /// A node in the syntax tree
    Node(QualifiedName),
}

impl Type {
    pub fn default_unit(&self) -> units::Unit {
        match self {
            Self::Length => units::Unit::Mm,
            Self::Angle => units::Unit::Rad,
            Self::List(t) => {
                if let Some(t) = t {
                    t.default_unit()
                } else {
                    units::Unit::None
                }
            }
            Self::Array(t, _) => t.default_unit(),
            _ => units::Unit::None,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "integer"),
            Self::Invalid => write!(f, "invalid"),
            Self::Scalar => write!(f, "scalar"),
            Self::String => write!(f, "string"),
            Self::Color => write!(f, "color"),
            Self::Length => write!(f, "length"),
            Self::Angle => write!(f, "angle"),
            Self::Vec2 => write!(f, "vec2"),
            Self::Vec3 => write!(f, "vec3"),
            Self::Bool => write!(f, "bool"),
            Self::List(t) => {
                if let Some(t) = t {
                    write!(f, "[{}]", t)
                } else {
                    write!(f, "[]")
                }
            }
            Self::Array(t, n) => write!(f, "[{}; {}]", t, n),
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
            Self::Node(qn) => write!(f, "{}", qn),
        }
    }
}

/// Trait for structs and expressions that have a type
pub trait Ty {
    fn ty(&self) -> Type;
}

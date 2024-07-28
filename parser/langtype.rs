use crate::units;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Type {
    /// Correspond to an uninitialized type, or an error
    #[default]
    Invalid,

    /// An f64
    Scalar,

    /// A string
    String,

    /// An RGBA color
    Color,

    /// A physical length, e.g. 4.0mm
    Length,

    /// An angle, e.g. 90°
    Angle,

    /// A boolean
    Bool,

    List(Option<Box<Type>>),

    Array(Box<Type>, usize),

    Tuple(Vec<Type>),
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
            Self::Invalid => write!(f, "invalid"),
            Self::Scalar => write!(f, "scalar"),
            Self::String => write!(f, "string"),
            Self::Color => write!(f, "color"),
            Self::Length => write!(f, "length"),
            Self::Angle => write!(f, "angle"),
            Self::Bool => write!(f, "bool"),
            Self::List(t) => {
                if let Some(t) = t {
                    write!(f, "[{}]", t)
                } else {
                    write!(f, "[]")
                }
            }
            Self::Array(t, n) => write!(f, "[{}; {}]", t, n),
            Self::Tuple(t) => {
                write!(f, "(")?;
                for (i, t) in t.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
        }
    }
}

/// Trait for structs and expressions that have a type
pub trait Ty {
    fn ty(&self) -> Type;
}

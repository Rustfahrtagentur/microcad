#[derive(Debug, Clone, Default)]
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

    List(Box<Type>),

    Array(Box<Type>, usize),

    Tuple(Vec<Type>),
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
            Self::List(t) => write!(f, "[{}]", t),
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

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
    Length,
    Angle,
    Bool,
}

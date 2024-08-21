use crate::language::*;

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

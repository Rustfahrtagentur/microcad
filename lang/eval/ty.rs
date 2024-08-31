use crate::r#type::Type;

/// Trait for structs and expressions that have a type
pub trait Ty {
    fn ty(&self) -> Type;
}

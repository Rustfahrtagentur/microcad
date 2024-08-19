mod errors;
mod list_type;
mod map_key_type;
mod map_type;
mod named_tuple_type;
mod r#type;
mod type_list;
mod unnamed_tuple_type;

pub use errors::*;
pub use list_type::*;
pub use map_key_type::*;
pub use map_type::*;
pub use named_tuple_type::*;
pub use r#type::*;
pub use type_list::*;
pub use unnamed_tuple_type::*;

/// Trait for structs and expressions that have a type
pub trait Ty {
    fn ty(&self) -> Type;
}

//! Type list type parser entity

use crate::r#type::*;

/// List of types
pub struct TypeList(Vec<Type>);

impl TypeList {
    /// Create new type list
    pub fn new(types: Vec<Type>) -> Self {
        Self(types)
    }

    pub fn common_type(&self) -> Option<Type> {
        let mut common_type = None;
        for ty in &self.0 {
            match common_type {
                None => common_type = Some(ty.clone()),
                Some(ref t) if t == ty => {}
                _ => return None,
            }
        }
        common_type
    }
}

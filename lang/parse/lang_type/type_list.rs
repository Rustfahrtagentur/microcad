// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Type list type parser entity

use crate::r#type::*;

/// List of types
pub struct TypeList(Vec<Type>);

impl TypeList {
    /// Create new type list
    pub fn new(types: Vec<Type>) -> Self {
        Self(types)
    }

    /// Check if all list items are of a common type
    pub fn common_type(&self) -> Option<Type> {
        if let Some(ty) = self.0.first() {
            if self.0[1..].iter().all(|t| t == ty) {
                return Some(ty.clone());
            }
        }
        None
    }
}

impl std::fmt::Debug for TypeList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::fmt::Display for TypeList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[test]
fn test_common_type() {
    let list = TypeList::new(vec![Type::Integer, Type::Integer]);
    assert_eq!(Some(Type::Integer), list.common_type());

    let list = TypeList::new(vec![Type::Integer, Type::Scalar]);
    assert_eq!(None, list.common_type());

    let list = TypeList::new(Vec::new());
    assert_eq!(None, list.common_type());
}

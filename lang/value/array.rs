// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Typed list of values evaluation entity

use crate::{ty::*, value::*};
use derive_more::{Deref, DerefMut};

/// List of values of the same type
#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Array {
    /// List of values
    #[deref]
    #[deref_mut]
    list: ValueList,
    ty: Type,
}

impl Array {
    /// Create new list
    pub fn new(list: ValueList, ty: Type) -> Self {
        Self { list, ty }
    }

    /// Fetch all values as `Vec<Value>`
    pub fn fetch(&self) -> Vec<Value> {
        self.list.iter().cloned().collect::<Vec<_>>()
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.list == other.list
    }
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        let list: ValueList = iter.into_iter().collect();
        let ty = list.types().common_type().expect("Common type");
        Self { ty, list }
    }
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{items}]",
            items = self
                .list
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl crate::ty::Ty for Array {
    fn ty(&self) -> Type {
        Type::List(ListType::new(self.ty.clone()))
    }
}

impl std::ops::Mul<Value> for Array {
    type Output = ValueResult;

    fn mul(self, rhs: Value) -> Self::Output {
        let mut values = Vec::new();
        for value in self.iter() {
            values.push((value.clone() * rhs.clone())?);
        }

        match self.ty {
            // List * Scalar or List * Integer
            Type::Quantity(_) | Type::Integer => Ok(Value::Array(Array::new(
                ValueList::new(values),
                rhs.ty().clone(),
            ))),
            _ => Err(ValueError::InvalidOperator("*".into())),
        }
    }
}

impl std::ops::Div<Value> for Array {
    type Output = ValueResult;

    fn div(self, rhs: Value) -> Self::Output {
        let mut values = Vec::new();
        for value in self.iter() {
            values.push((value.clone() / rhs.clone())?);
        }

        match self.ty {
            // List / Scalar or List / Integer
            Type::Quantity(_) | Type::Integer => Ok(Value::Array(Array::new(
                ValueList::new(values),
                Type::Quantity(QuantityType::Scalar),
            ))),
            _ => Err(ValueError::InvalidOperator("/".into())),
        }
    }
}

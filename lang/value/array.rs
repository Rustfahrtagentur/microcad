// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Typed list of values evaluation entity

use crate::{ty::*, value::*};
use derive_more::{Deref, DerefMut};

/// Collection of values of the same type.
#[derive(Clone, Debug, Deref, DerefMut, serde::Serialize, serde::Deserialize)]
pub struct Array {
    /// List of values
    #[deref]
    #[deref_mut]
    items: ValueList,
    /// Element type.
    ty: Type,
}

impl Array {
    /// Create new list
    pub fn new(items: ValueList, ty: Type) -> Self {
        Self { items, ty }
    }

    /// Fetch all values as `Vec<Value>`
    pub fn fetch(&self) -> Vec<Value> {
        self.items.iter().cloned().collect::<Vec<_>>()
    }
}

impl PartialEq for Array {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.items == other.items
    }
}

impl IntoIterator for Array {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl FromIterator<Value> for Array {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        let list: ValueList = iter.into_iter().collect();
        let ty = list.types().common_type().expect("Common type");
        Self { ty, items: list }
    }
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{items}]",
            items = self
                .items
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl crate::ty::Ty for Array {
    fn ty(&self) -> Type {
        Type::Array(Box::new(self.ty.clone()))
    }
}

impl std::ops::Add<Value> for Array {
    type Output = ValueResult;

    fn add(self, rhs: Value) -> Self::Output {
        if rhs.ty() == self.ty {
            Ok(Value::Array(Self::new(
                ValueList::new(
                    self.items
                        .iter()
                        .map(|value| value.clone() + rhs.clone())
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                self.ty,
            )))
        } else {
            Err(ValueError::InvalidOperator("+".into()))
        }
    }
}

impl std::ops::Sub<Value> for Array {
    type Output = ValueResult;

    fn sub(self, rhs: Value) -> Self::Output {
        if rhs.ty() == self.ty {
            Ok(Value::Array(Self::new(
                ValueList::new(
                    self.items
                        .iter()
                        .map(|value| value.clone() - rhs.clone())
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                self.ty,
            )))
        } else {
            Err(ValueError::InvalidOperator("-".into()))
        }
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
                self.ty * rhs.ty().clone(),
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

        match (&self.ty, rhs.ty()) {
            // Integer / Integer => Scalar
            (Type::Integer, Type::Integer) => Ok(Value::Array(Array::new(
                ValueList::new(values),
                Type::scalar(),
            ))),
            (Type::Quantity(_), rty) => Ok(Value::Array(Array::new(
                ValueList::new(values),
                self.ty / rty.clone(),
            ))),
            _ => Err(ValueError::InvalidOperator("/".into())),
        }
    }
}

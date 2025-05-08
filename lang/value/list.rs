// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Typed list of values evaluation entity

use crate::{syntax::*, ty::*, value::*};

/// List of values of the same type
#[derive(Clone, Debug)]
pub struct List {
    /// List of values
    list: ValueList,
    ty: Type,
}

impl List {
    /// Create new list
    pub fn new(list: ValueList, ty: Type) -> Self {
        Self { list, ty }
    }

    /// Fetch all values as `Vec<Value>`
    pub fn fetch(&self) -> Vec<Value> {
        self.list.iter().cloned().collect::<Vec<_>>()
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.list == other.list
    }
}

impl std::ops::Deref for List {
    type Target = ValueList;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl std::ops::DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}

impl IntoIterator for List {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

impl std::fmt::Display for List {
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

impl crate::ty::Ty for List {
    fn ty(&self) -> Type {
        Type::List(ListType::new(self.ty.clone()))
    }
}

impl std::ops::Mul<Value> for List {
    type Output = ValueResult;

    fn mul(self, rhs: Value) -> Self::Output {
        let mut values = Vec::new();
        for value in self.iter() {
            values.push((value.clone() * rhs.clone())?);
        }

        match self.ty {
            // List * Scalar or List * Integer
            Type::Scalar | Type::Integer | Type::Length | Type::Area | Type::Angle => Ok(
                Value::List(List::new(ValueList::new(values), rhs.ty().clone())),
            ),
            _ => Err(ValueError::InvalidOperator("*".into())),
        }
    }
}

impl std::ops::Div<Value> for List {
    type Output = ValueResult;

    fn div(self, rhs: Value) -> Self::Output {
        let mut values = Vec::new();
        for value in self.iter() {
            values.push((value.clone() / rhs.clone())?);
        }

        match self.ty {
            // List / Scalar or List / Integer
            Type::Scalar | Type::Integer | Type::Length | Type::Area | Type::Angle => {
                Ok(Value::List(List::new(ValueList::new(values), Type::Scalar)))
            }
            _ => Err(ValueError::InvalidOperator("/".into())),
        }
    }
}

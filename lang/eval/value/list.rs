// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Typed list of values evaluation entity

use crate::{eval::*, r#type::*, src_ref::*};

/// List of values of the same type
#[derive(Clone, Debug, PartialEq)]
pub struct List {
    /// List of values
    list: ValueList,
    ty: Type,
    src_ref: SrcRef,
}

impl List {
    /// Create new list
    pub fn new(list: ValueList, ty: Type, src_ref: SrcRef) -> Self {
        Self { list, ty, src_ref }
    }

    /// Fetch all values as Vec<Value>
    pub fn fetch(&self) -> Vec<Value> {
        self.list.iter().cloned().collect::<Vec<_>>()
    }
}

impl SrcReferrer for List {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
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

impl Ty for List {
    fn ty(&self) -> Type {
        Type::List(crate::parse::ListType::new(self.ty.clone()))
    }
}

impl std::ops::Mul<Value> for List {
    type Output = Result<List>;

    fn mul(self, rhs: Value) -> Self::Output {
        let mut values = Vec::new();
        for value in self.iter() {
            values.push((value.clone() * rhs.clone())?);
        }

        match (self.ty, rhs) {
            (Type::Scalar, rhs) => Ok(List::new(
                ValueList::new(values, self.src_ref.clone()),
                rhs.ty().clone(),
                self.src_ref.clone(),
            )),
            _ => Err(EvalError::InvalidOperator("*".into())),
        }
    }
}

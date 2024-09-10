// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, parse::*, r#type::*, src_ref::*};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueList(Refer<Vec<Value>>);

impl ValueList {
    pub fn new(list: Vec<Value>, src_ref: SrcRef) -> Self {
        Self(Refer::new(list, src_ref))
    }

    pub fn add_unit_to_unitless_types(
        &mut self,
        unit: Unit,
    ) -> std::result::Result<(), ValueError> {
        for value in self.0.iter_mut() {
            value.add_unit_to_unitless_types(unit)?;
        }
        Ok(())
    }

    pub fn types(&self) -> TypeList {
        TypeList::new(
            self.0
                .iter()
                .map(|v| v.ty())
                .collect::<Vec<Type>>()
                .into_iter()
                .collect(),
        )
    }
}

impl std::ops::Deref for ValueList {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SrcReferrer for ValueList {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl IntoIterator for ValueList {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.value.into_iter()
    }
}

impl std::iter::FromIterator<Value> for ValueList {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        let vec = Vec::from_iter(iter);
        let src_ref = SrcRef::from_vec(&vec);
        ValueList(Refer::new(vec, src_ref))
    }
}


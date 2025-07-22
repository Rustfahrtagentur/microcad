// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object properties.

use crate::{diag::PushDiag, eval::*, src_ref::SrcRef, syntax::*, value::*};
use derive_more::{Deref, DerefMut};
use std::collections::BTreeMap;

/// A list of object properties.
///
/// It is required that properties are always sorted by their id.
#[derive(Clone, Default, Debug, Deref, DerefMut)]
pub struct ObjectProperties(BTreeMap<Identifier, Value>);

impl ObjectProperties {
    /// Create new object properties from a [`ParameterValueList`] and an [`Tuple`].
    pub fn from_parameters_and_arguments(
        parameters: &ParameterValueList,
        arguments: &Tuple,
    ) -> Self {
        parameters
            .iter()
            .map(|(id, param)| {
                (
                    id.clone(),
                    match arguments.by_id(id) {
                        Some(value) => value.clone(),
                        None => param.default_value.clone().unwrap_or(Value::None),
                    },
                )
            })
            .collect()
    }
}

impl FromIterator<(Identifier, Value)> for ObjectProperties {
    fn from_iter<T: IntoIterator<Item = (Identifier, Value)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Eval<()> for ObjectProperties {
    fn eval(&self, context: &mut Context) -> EvalResult<()> {
        if self.is_valid() {
            for (id, value) in self.iter() {
                context.set_local_value(id.clone(), value.clone())?;
            }
        } else {
            context.error(
                SrcRef(None), // Hmm, where to get the src ref here?
                EvalError::UninitializedProperties(self.get_ids_of_uninitialized().into()),
            )?;
        }

        Ok(())
    }
}

impl ObjectProperties {
    /// Test if each property has a value.
    pub fn is_valid(&self) -> bool {
        self.0.iter().all(|(_, value)| !value.is_invalid())
    }

    /// Get mutable value at id.
    pub fn get_value_mut(&mut self, id: &Identifier) -> Option<&mut Value> {
        self.0.get_mut(id)
    }

    /// Insert property.
    pub fn insert(&mut self, id: Identifier, value: Value) {
        self.0.insert(id, value);
    }

    /// Get all ids where `value == Value::None`.
    pub fn get_ids_of_uninitialized(&self) -> Vec<Identifier> {
        self.0
            .iter()
            .filter_map(|(id, value)| {
                if value.is_invalid() {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get value of property with given id or `None` if not found.
    pub fn get_property(&self, id: &Identifier) -> Option<&Value> {
        self.0.get(id)
    }

    /// Set or create property with given id and value-
    pub fn set_property(&mut self, id: Identifier, value: Value) {
        self.0.insert(id, value);
    }
}

impl std::fmt::Display for ObjectProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ObjectProperties:")?;
        for (id, value) in self.0.iter() {
            writeln!(f, "\t{id:?} = {value:?}")?;
        }

        Ok(())
    }
}

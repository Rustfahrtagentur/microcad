// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object properties.

use std::collections::BTreeMap;

use crate::{GetPropertyValue, diag::PushDiag, eval::*, src_ref::SrcRef, syntax::*, value::*};

/// A list of object properties.
///
/// It is required that properties are always sorted by their id.
#[derive(Clone, Default, Debug)]
pub struct ObjectProperties(BTreeMap<Identifier, Value>);

impl ObjectProperties {
    /// Create new object properties from a [`ParameterValueList`] and an [`ArgumentMap`].
    pub fn from_parameters_and_arguments(
        parameters: &ParameterValueList,
        arguments: &ArgumentMap,
    ) -> Self {
        let mut props = ObjectProperties::default();

        for parameter in parameters.iter() {
            props.insert(
                parameter.id.clone(),
                match &parameter.default_value {
                    Some(value) => value.clone(),
                    None => arguments
                        .get_value(&parameter.id)
                        .unwrap_or(&Value::None)
                        .clone(),
                },
            );
        }

        props
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

impl std::ops::Deref for ObjectProperties {
    type Target = std::collections::BTreeMap<Identifier, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ObjectProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl GetPropertyValue for ObjectProperties {
    fn get_property_value(&self, id: &Identifier) -> Value {
        self.0.get(id).cloned().unwrap_or_default()
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

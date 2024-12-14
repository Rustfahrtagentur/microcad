// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument map evaluation entity

use crate::{eval::*, parse::Combinations, src_ref::*};

/// Map of arguments
#[derive(Clone, Debug, Default)]
pub struct ArgumentMap(Refer<std::collections::HashMap<Id, Value>>);

impl ArgumentMap {
    /// Create empty argument map
    pub fn new(src_ref: SrcRef) -> Self {
        Self(Refer::new(std::collections::HashMap::new(), src_ref))
    }

    /// Fetch an argument by name
    pub fn get_value<'a, T>(&'a self, name: &str) -> T
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        self.0
            .get(name)
            .expect("no name found")
            .try_into()
            .expect("cannot convert argument value")
    }
}

impl SrcReferrer for ArgumentMap {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for ArgumentMap {
    type Target = std::collections::HashMap<Id, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ArgumentMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// An argument map for parameter multiplicity.
///
/// In the combination map, every value can be a single or multi coefficient.
/// Let's assume, you have a `module a(r: scalar) {}`:
/// * If you call `a(4.0)`, `a` will be stored as a single coefficient, because we passed a single scalar.
/// * If you call `a([2.0, 4.0])`, `a` will be stored as a multi coefficient, because we passed a list of scalars.
#[derive(Default)]
pub struct MultiArgumentMap(crate::parse::call::CombinationMap<Value>);

impl MultiArgumentMap {
    /// Insert a multi-value coefficient
    pub fn insert_multi(&mut self, name: Id, value: Vec<Value>) {
        self.0
            .insert(name, crate::parse::call::Coefficient::Multi(value));
    }

    /// Insert a single-value coefficient
    pub fn insert_single(&mut self, name: Id, value: Value) {
        self.0
            .insert(name, crate::parse::call::Coefficient::Single(value));
    }

    /// Return an iterator over all combinations
    pub fn combinations(&self) -> Combinations<Value> {
        Combinations::new(&self.0)
    }

    /// Check if the argument map contains a key
    pub fn contains_key(&self, key: &Id) -> bool {
        self.0.contains_key(key)
    }

    /// Insert a value into the map and remove `parameter_value` from the list
    pub fn insert_and_remove_from_parameters(
        &mut self,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> TypeCheckResult {
        let result = parameter_value.type_check(&value.ty());
        let name = &parameter_value.name;
        match result {
            TypeCheckResult::MultiMatch => match &value {
                Value::List(l) => {
                    parameter_values.remove(name);
                    self.insert_multi(name.clone(), l.fetch());
                    result
                }
                value => panic!("Expected list type, got {}", value.ty()),
            },
            TypeCheckResult::SingleMatch => {
                parameter_values.remove(&name);
                self.insert_single(name.clone(), value);
                result
            }
            _ => result,
        }
    }
}

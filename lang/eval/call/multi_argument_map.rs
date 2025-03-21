// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Multi argument map evaluation entity

use crate::{eval::*, parse::Combinations};

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
}

impl ArgumentMatch for MultiArgumentMap {
    /// Insert a value into the map and remove `parameter_value` from the list
    fn insert_and_remove_from_parameters(
        &mut self,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> EvalResult<TypeCheckResult> {
        let result = parameter_value.type_check(&value.ty());
        let name = &parameter_value.name;
        match result {
            TypeCheckResult::MultiMatch => match &value {
                Value::List(l) => {
                    parameter_values.remove(name);
                    self.insert_multi(name.clone(), l.fetch());
                    Ok(result)
                }
                value => Err(EvalError::ExpectedIterable(value.ty().clone())),
            },
            TypeCheckResult::SingleMatch => {
                parameter_values.remove(name);
                self.insert_single(name.clone(), value);
                Ok(result)
            }
            _ => Ok(result),
        }
    }
}

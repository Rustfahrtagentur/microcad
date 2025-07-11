// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Multi argument map evaluation entity

use crate::{eval::*, ty::*, value::*};

/// An argument map for parameter multiplicity.
///
/// In the combination map, every value can be a single or multi coefficient.
/// Let's assume, you have a `part a(r: scalar) {}`:
/// * If you call `a(4.0)`, `a` will be stored as a single coefficient, because we passed a single scalar.
/// * If you call `a([2.0, 4.0])`, `a` will be stored as a multi coefficient, because we passed a list of scalars.
#[derive(Default)]
pub struct MultiArgumentMap(Refer<CombinationMap<Value>>);

impl MultiArgumentMap {
    /// Insert a multi-value coefficient
    pub fn insert_multi(&mut self, id: Identifier, value: Vec<Value>) {
        self.0.insert(id, Coefficient::Multi(value));
    }

    /// Insert a single-value coefficient
    pub fn insert_single(&mut self, id: Identifier, value: Value) {
        self.0.insert(id, Coefficient::Single(value));
    }

    /// Return an iterator over all combinations
    pub fn combinations(&self) -> Combinations<Value> {
        Combinations::new(&self.0)
    }
}

impl SrcReferrer for MultiArgumentMap {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl ArgumentMatch for MultiArgumentMap {
    fn new(src_ref: SrcRef) -> Self {
        Self(Refer::new(std::collections::HashMap::new(), src_ref))
    }

    fn insert_and_remove_from_parameters(
        &mut self,
        id: &Identifier,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> EvalResult<TypeCheckResult> {
        let result = parameter_value.type_check(&value.ty());
        match result {
            TypeCheckResult::MultiMatch => match &value {
                Value::Array(l) => {
                    parameter_values.remove(id);
                    self.insert_multi(id.clone(), l.fetch());
                    Ok(result)
                }
                value => Err(EvalError::ExpectedIterable(value.ty().clone())),
            },
            TypeCheckResult::SingleMatch => {
                parameter_values.remove(id);
                self.insert_single(id.clone(), value);
                Ok(result)
            }
            _ => Ok(result),
        }
    }
}

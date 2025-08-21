// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter value list evaluation entity

use crate::{eval::*, value::*};
use compact_str::CompactStringExt;
use derive_more::Deref;

/// List of parameter values
#[derive(Clone, Debug, Default, Deref)]
pub struct ParameterValueList(std::collections::HashMap<Identifier, ParameterValue>);

impl ParameterValueList {
    /// Push parameter value
    pub fn insert(
        &mut self,
        id: Identifier,
        parameter: ParameterValue,
    ) -> std::result::Result<(), ValueError> {
        assert!(!id.is_empty(), "expecting valid id");
        if self.0.contains_key(&id) {
            return Err(ValueError::DuplicateParameter(id.clone()));
        }
        self.0.insert(id, parameter);
        Ok(())
    }
}

impl std::fmt::Display for ParameterValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let mut v = self
                .0
                .iter()
                .map(|(id, p)| format!("{id}: {p}"))
                .collect::<Vec<_>>();
            v.sort();
            v.join_compact(", ")
        })
    }
}

impl<I, P> FromIterator<(I, P)> for ParameterValueList
where
    I: Into<Identifier>,
    P: Into<ParameterValue>,
{
    fn from_iter<T: IntoIterator<Item = (I, P)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|(i, p)| (i.into(), p.into()))
                .collect(),
        )
    }
}

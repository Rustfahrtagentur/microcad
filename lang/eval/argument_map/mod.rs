// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument map

mod argument_match;
mod multi_argument_map;
mod multiplicity;

pub use argument_match::*;
pub use multi_argument_map::*;
pub use multiplicity::*;

use crate::{eval::*, src_ref::*, value::*};

/// Map of named arguments
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ArgumentMap {
    map: std::collections::HashMap<Identifier, Value>,
    src_ref: SrcRef,
}

impl ArgumentMap {
    /// Fetch an argument by name as `&str`.
    ///
    /// This method does not provide error handling and is supposed to be used for built-ins.
    pub fn get<'a, T>(&'a self, id: &Identifier) -> T
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        self.map
            .get(id)
            .expect("no name found")
            .try_into()
            .expect("cannot convert argument value")
    }

    /// Fetch an argument's value by name.
    pub fn get_value(&self, id: &Identifier) -> Option<&Value> {
        self.map.get(id)
    }

    /// Convert ArgumentMap into symbol map.
    pub fn into_symbols(self) -> SymbolMap {
        let mut symbols = SymbolMap::new();
        for (id, arg) in self.map.iter() {
            symbols.insert_node(
                id.clone(),
                Symbol::new(SymbolDefinition::Argument(id.clone(), arg.clone()), None),
            )
        }
        symbols
    }

    /// Print the [`ArgumentMap`] as one line, truncated if `max length > 0`.
    pub fn to_one_line_string(&self, max_length: Option<usize>) -> String {
        let mut sorted: Vec<_> = self.map.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));

        let mut output = String::new();

        let max_length = max_length.unwrap_or_default();

        for (i, (k, v)) in sorted.iter().enumerate() {
            if i != 0 {
                output.push_str(", ");
            }
            output.push_str(&format!("{}: {} = {}", k, v.ty(), v));

            if output.len() > max_length && max_length > 0 {
                output = output.chars().take(max_length).collect::<String>();
                output += "...";
                break;
            }
        }

        output
    }
}

impl SrcReferrer for ArgumentMap {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::ops::Deref for ArgumentMap {
    type Target = std::collections::HashMap<Identifier, Value>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl std::ops::DerefMut for ArgumentMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<I> FromIterator<(I, Value)> for ArgumentMap
where
    I: Into<Identifier>,
{
    fn from_iter<T: IntoIterator<Item = (I, Value)>>(iter: T) -> Self {
        Self {
            map: iter.into_iter().map(|(i, v)| (i.into(), v)).collect(),
            src_ref: SrcRef(None),
        }
    }
}

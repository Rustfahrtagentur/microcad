// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Parameter multiplicity implementation.

use crate::{eval::*, value::*};

/// An enum to distinguish single-value and multi-value coefficients
#[derive(Clone, Debug)]
pub enum Coefficient<T> {
    /// A single value
    Single(T),
    /// A multi value
    Multi(Vec<T>),
}

impl<T> Coefficient<T> {
    /// Number of items in the coefficient
    pub fn count(&self) -> usize {
        match self {
            Self::Single(_) => 1,
            Self::Multi(v) => v.len(),
        }
    }
}

impl<T> std::ops::Index<usize> for Coefficient<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Single(t) => t,
            Self::Multi(v) => &v[index],
        }
    }
}

/// Iterator over combinations
pub struct Combinations<T> {
    /// The actual value data. Stored in a Vec instead of a HashMap
    data: Vec<Coefficient<T>>,
    /// The indices of the iterator. Stored in a Vec instead of a HashMap
    indices: Vec<usize>,
    /// Used to map Id to data indices
    data_indices: std::collections::BTreeMap<Identifier, usize>,
    /// Flag to tell if the iterator is finished
    done: bool,
}

/// A map over combinations
pub type CombinationMap<T> = std::collections::HashMap<Identifier, Coefficient<T>>;

impl<T> Combinations<T>
where
    T: Clone,
{
    /// Create a new Combinations iterator
    pub fn new(data: &CombinationMap<T>) -> Self {
        let mut ids_sorted: Vec<_> = data.keys().cloned().collect();
        ids_sorted.sort();
        let keys_sorted: Vec<_> = (0..ids_sorted.len()).collect();

        let indices = ids_sorted.iter().map(|_| 0).collect();

        let data_indices = ids_sorted
            .iter()
            .zip(keys_sorted)
            .map(|(id, key)| (id.clone(), key))
            .collect();

        let data = ids_sorted
            .iter()
            .map(|id| data.get(id).expect("Coefficient with id").clone())
            .collect();

        Combinations {
            data,
            indices,
            data_indices,
            done: false,
        }
    }

    /// Advance the index counters by one step
    fn advance_indices(&mut self) {
        for (_, index) in self.data_indices.iter() {
            self.indices[*index] = *self.indices.get(*index).expect("Index") + 1;

            let count = self.data[*index].count();
            if self.indices[*index] < count {
                break;
            }
            self.indices[*index] = 0;

            if *index == self.data_indices.len() - 1 {
                self.done = true;
            }
        }
    }
}

impl Iterator for Combinations<Value> {
    type Item = ArgumentMap;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.data_indices.is_empty() {
            self.done = true;
            return Some(ArgumentMap::default());
        }

        // Create the current combination based on the current indices
        let values: Vec<_> = self
            .data
            .iter()
            .zip(&self.indices)
            .map(|(coeff, index)| coeff[*index].clone())
            .collect();

        self.advance_indices();

        let mut args = ArgumentMap::default();
        for (key, index) in &self.data_indices {
            args.insert(key.clone(), values[*index].clone());
        }

        Some(args)
    }
}

#[test]
fn call_parameter_multiplicity() {
    let data = std::collections::HashMap::from([
        (
            "0".into(),
            Coefficient::Multi([1, 2].iter().map(|i| Value::Integer(*i)).collect()),
        ),
        (
            "1".into(),
            Coefficient::Multi([10, 20, 30].iter().map(|i| Value::Integer(*i)).collect()),
        ),
        (
            "2".into(),
            Coefficient::Multi([100, 200].iter().map(|i| Value::Integer(*i)).collect()),
        ),
        ("3".into(), Coefficient::Single(Value::Integer(20))),
    ]);

    let combinations = Combinations::new(&data);
    assert!(!&combinations.done);

    let mut count = 0;
    for combination in combinations {
        let mut keys = combination.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        let items: Vec<(&Identifier, i64)> = keys
            .iter()
            .map(|key| {
                (
                    key,
                    combination[key].clone().try_into().expect("test error"),
                )
            })
            .collect();
        log::trace!("{:?}", items);
        count += 1;
    }

    assert_eq!(
        count,
        2 * 3 * 2,
        "There must be 12 combinations, but only {count} iterated."
    );

    // Test empty map
    let data = std::collections::HashMap::new();

    let combinations = Combinations::new(&data);
    let mut count = 0;

    for _ in combinations {
        count += 1;
    }

    assert_eq!(
        count, 1,
        "There must be 1 combination, but only {count} iterated."
    );
}

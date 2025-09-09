// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree module

use crate::{model::*, src_ref::*};
use derive_more::{Deref, DerefMut};
use microcad_core::BooleanOp;

/// Model multiplicities.
#[derive(Debug, Default, Clone, PartialEq, Deref, DerefMut)]
pub struct Models(Vec<Model>);

impl Models {
    /// Returns the first model if there is exactly one model in the list.
    pub fn single_model(&self) -> Option<Model> {
        match self.0.len() {
            1 => self.0.first().cloned(),
            _ => None,
        }
    }

    /// Convert the models into a multiplicity node.
    pub fn to_multiplicity(&self, src_ref: SrcRef) -> Model {
        match self.single_model() {
            Some(model) => model,
            None => ModelBuilder::new(Element::Multiplicity, src_ref)
                .add_children(self.clone())
                .expect("No error")
                .build(),
        }
    }

    /// Check if all models contain an operation element.
    pub fn is_operation(&self) -> bool {
        self.iter().all(Model::is_operation)
    }

    /// A union operation model for this collection.
    pub fn union(&self) -> Model {
        self.boolean_op(microcad_core::BooleanOp::Union)
    }

    /// Return an boolean operation model for this collection.
    pub fn boolean_op(&self, op: BooleanOp) -> Model {
        match self.single_model() {
            Some(model) => model,
            None => ModelBuilder::new(Element::BuiltinWorkpiece(op.into()), SrcRef(None))
                .add_children(
                    [ModelBuilder::new(Element::Group, SrcRef(None))
                        .add_children(self.clone())
                        .expect("No error")
                        .build()]
                    .into_iter()
                    .collect(),
                )
                .expect("No error")
                .build(),
        }
    }

    /// Merge two lists of [`Models`] into one by concatenation.
    /// TODO: Use iterators!
    pub fn merge(lhs: Models, rhs: Models) -> Self {
        lhs.iter().chain(rhs.iter()).cloned().collect()
    }

    /// Filter the models by source file.
    pub fn filter_by_source_hash(&self, source_hash: u64) -> Models {
        self.iter()
            .filter(|model| source_hash == model.source_hash())
            .cloned()
            .collect()
    }

    /// Deduce output type from models.
    pub fn deduce_output_type(&self) -> OutputType {
        self.iter().map(|model| model.deduce_output_type()).fold(
            OutputType::NotDetermined,
            // TODO: weird naming
            |output_type, model_output_type| output_type.merge(&model_output_type),
        )
    }
}

impl From<Vec<Model>> for Models {
    fn from(value: Vec<Model>) -> Self {
        Self(value)
    }
}

impl From<Option<Model>> for Models {
    fn from(value: Option<Model>) -> Self {
        Self(value.into_iter().collect())
    }
}

impl std::fmt::Display for Models {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().try_for_each(|model| model.fmt(f))
    }
}

impl FromIterator<Model> for Models {
    fn from_iter<T: IntoIterator<Item = Model>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl TreeDisplay for Models {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        self.iter().try_for_each(|child| child.tree_print(f, depth))
    }
}

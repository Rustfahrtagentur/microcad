// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model tree module

use crate::{model::*, resolve::*, src_ref::*, syntax::SourceFile};
use derive_more::{Deref, DerefMut};
use microcad_core::BooleanOp;

/// Model multiplicities.
#[derive(
    Debug, Default, Clone, PartialEq, Deref, DerefMut, serde::Serialize, serde::Deserialize,
)]
pub struct Models(Vec<Model>);

impl Models {
    /// Returns the first model if there is exactly one model in the list.
    pub fn single_model(&self) -> Option<Model> {
        match self.0.len() {
            1 => self.0.first().cloned(),
            _ => None,
        }
    }

    /// Returns a property of the included models.
    pub fn fetch_property(&self, id: &Identifier) -> Option<Value> {
        if let Some(model) = self.single_model() {
            model.borrow().get_property(id).cloned()
        } else {
            Some(
                self.0
                    .iter()
                    .filter_map(|model| {
                        let model_ = model.borrow();
                        model_.get_property(id).cloned()
                    })
                    .collect(),
            )
        }
    }

    /// Nest models in self.
    pub fn nest(self, models: &Models) -> Self {
        self.iter().for_each(|new_parent| {
            models.iter().for_each(|model| {
                model.detach();

                // Handle children marker.
                // If we have found a children marker model, use it's parent as
                // new parent model.
                let new_parent = match &new_parent.find_children_placeholder() {
                    Some(children_marker) => {
                        let parent = &children_marker
                            .borrow()
                            .parent
                            .clone()
                            .expect("Must have a parent");
                        children_marker.detach(); // Remove children marker from tree
                        parent.clone()
                    }
                    None => new_parent.clone(),
                };

                new_parent.append(model.make_deep_copy());
            });
        });

        self
    }

    /// Nest a Vec of model multiplicities
    ///
    /// * `model_stack`: A list of model lists.
    ///
    /// The reference to the first stack element will be returned.
    ///
    /// Assume, our model stack `Vec<Vec<Model>>` has for lists `a`, `b`, `c`, `d`:
    /// ```ignore
    /// let models = vec![
    ///     vec![obj("a0"), obj("a1")],
    ///     vec![obj("b0")],
    ///     vec![obj("c0"), obj("c1"), obj("c2")],
    ///     vec![obj("d0")],
    /// ];
    /// ```
    ///
    /// This should result in following model multiplicity:
    /// a0
    ///   b0
    ///     c0
    ///       d0
    ///     c1
    ///       d0
    ///     c2
    ///       d0
    /// a1
    ///   b0
    ///     c0
    ///       d0
    ///     c1
    ///       d0
    ///     c2
    ///       d0
    pub fn from_nested_items(items: &[Models]) -> Self {
        match items.len() {
            0 => panic!("Model stack must not be empty"),
            1 => {}
            n => {
                (1..n)
                    .rev()
                    .map(|i| (&items[i], &items[i - 1]))
                    .for_each(|(prev, curr)| {
                        // Insert a copy of each element `model` from `prev`
                        // as child to each element `new_parent` in `curr`
                        curr.iter().for_each(|new_parent| {
                            prev.iter().for_each(|model| {
                                model.detach();

                                // Handle children marker.
                                // If we have found a children marker model, use it's parent as
                                // new parent model.
                                let new_parent = match &new_parent.find_children_placeholder() {
                                    Some(children_marker) => {
                                        let parent = &children_marker
                                            .borrow()
                                            .parent
                                            .clone()
                                            .expect("Must have a parent");
                                        children_marker.detach(); // Remove children marker from tree
                                        parent.clone()
                                    }
                                    None => new_parent.clone(),
                                };

                                new_parent.append(model.make_deep_copy());
                            });
                        });
                    });
            }
        }

        items[0].clone()
    }

    /// A union operation model for this collection.
    pub fn union(&self) -> Model {
        self.boolean_op(microcad_core::BooleanOp::Union)
    }

    /// Return an boolean operation model for this collection.
    pub fn boolean_op(&self, op: BooleanOp) -> Model {
        match self.single_model() {
            Some(model) => model,
            None => ModelBuilder::new_operation(op)
                .add_children(
                    [ModelBuilder::new_group()
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

    /// Set the information about the creator for all models.
    ///
    /// See [`Model::set_creator`] for more info.
    pub fn set_creator(&self, creator: Symbol, call_src_ref: SrcRef) {
        self.iter().for_each(|model| {
            model
                .borrow_mut()
                .set_creator(creator.clone(), call_src_ref.clone())
        })
    }

    /// Filter the models by source file.
    pub fn filter_by_source_file(&self, source_file: &std::rc::Rc<SourceFile>) -> Models {
        self.iter()
            .filter(|model| match model.find_source_file() {
                Some(other) => source_file.hash == other.hash,
                None => false,
            })
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

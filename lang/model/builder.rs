// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model builder.

use crate::{eval::*, model::*, src_ref::SrcRef};

/// A builder pattern to build models.
#[derive(Default)]
pub struct ModelBuilder {
    root: ModelInner,
    /// Properties of the model.
    pub properties: Properties,
    /// Children models of this model.
    pub children: Models,
}

/// `ModelBuilder` creation.
///
/// All methods in this `impl` block are used to create a new model builder with a specific [`Element`] type.
impl ModelBuilder {
    /// Create a new model with an element.
    pub fn new(element: Element, src_ref: SrcRef) -> Self {
        Self {
            root: ModelInner::new(element, src_ref),
            ..Default::default()
        }
    }

    /// Add multiple children to the model if it matches.
    pub fn add_children(mut self, mut children: Models) -> EvalResult<Self> {
        self.children.append(&mut children);
        Ok(self)
    }

    /// Set model attributes.
    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.root.attributes = attributes;
        self
    }

    /// Set model properties.
    pub fn properties(mut self, properties: Properties) -> Self {
        self.properties = properties;
        self
    }

    /// Build a [`Model`].
    pub fn build(mut self) -> Model {
        if let Element::Workpiece(workpiece) = &mut self.root.element.value {
            workpiece.add_properties(self.properties);
        }

        let model = Model::new(self.root.into());
        model.append_children(self.children);
        model.deduce_output_type();
        model
    }
}

impl std::fmt::Display for ModelBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.properties)
    }
}

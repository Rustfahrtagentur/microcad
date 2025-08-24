// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model builder.

use microcad_core::{Geometry2D, Geometry3D};

use crate::{eval::*, model::*, rc::*, syntax::*};

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
    /// Create a new group from a body `{ ... }`.
    pub fn new_group() -> Self {
        Self {
            root: Default::default(),
            ..Default::default()
        }
    }

    /// Create a new workpiece.
    ///
    /// This function is used when a call to a workbench definition is evaluated.
    pub fn new_workpiece(workpiece_kind: WorkbenchKind) -> Self {
        Self {
            root: ModelInner::new(Element::Workpiece(workpiece_kind.into())),
            ..Default::default()
        }
    }

    /// Create a new children placeholder
    pub fn new_children_placeholder() -> Self {
        Self {
            root: ModelInner::new(Element::ChildrenMarker),
            ..Default::default()
        }
    }

    pub fn new_builtin_workpiece(builtin_workpiece: BuiltinWorkpiece) -> Self {
        Self {
            root: ModelInner::new(Element::BuiltinWorkpiece(builtin_workpiece)),
            ..Default::default()
        }
    }
}

impl ModelBuilder {
    /// Add multiple children to the model if it matches.
    pub fn add_children(mut self, mut children: Models) -> EvalResult<Self> {
        self.children.append(&mut children);
        Ok(self)
    }

    /// Set object origin.
    pub fn origin(mut self, origin: Origin) -> Self {
        self.root.origin = origin;
        self
    }

    /// Set object attributes.
    pub fn attributes(mut self, attributes: Attributes) -> Self {
        self.root.attributes = attributes;
        self
    }

    /// Set object properties.
    pub fn properties(mut self, properties: Properties) -> Self {
        log::trace!("Properties:\n{properties}");
        self.properties = properties;
        self
    }

    /// Build a [`Model`].
    pub fn build(mut self) -> Model {
        if let Element::Workpiece(workpiece) = &mut self.root.element {
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

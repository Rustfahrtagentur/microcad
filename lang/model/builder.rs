// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model builder.

use microcad_core::{Geometry2D, Geometry3D};

use crate::{eval::*, model::*, rc::*, syntax::*};

/// A builder pattern to build models
#[derive(Default)]
pub struct ModelBuilder {
    root: ModelInner,
    /// Properties to add to the model if it is an [`Object`]
    pub properties: Properties,
    /// Children to add to this model.
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

    /// New 2D primitive.
    pub fn new_2d_primitive(geometry: std::rc::Rc<Geometry2D>) -> Self {
        Self {
            root: geometry.into(),
            ..Default::default()
        }
    }

    /// New 3D primitive.
    pub fn new_3d_primitive(geometry: std::rc::Rc<Geometry3D>) -> Self {
        Self {
            root: geometry.into(),
            ..Default::default()
        }
    }

    /// New transform.
    pub fn new_transform(transform: AffineTransform) -> Self {
        Self {
            root: transform.into(),
            ..Default::default()
        }
    }

    /// New operation.
    pub fn new_operation<T: Operation + 'static>(operation: T) -> Self {
        Self {
            root: ModelInner::new(Element::Operation(Rc::new(operation))),
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

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model builder.

use microcad_core::{Geometry2D, Geometry3D};

use crate::{eval::*, model::*, rc::*, src_ref::*, syntax::*, value::*};

/// A builder pattern to build models
#[derive(Default)]
pub struct ModelBuilder {
    root: ModelInner,
    /// Properties to add to the model if it is an [`Object`]
    pub properties: ObjectProperties,
    /// Children to add to this model.
    pub children: Models,

    /// The output type of this model.
    output: OutputType,
}

/// `ModelBuilder` creation.
///
/// All methods in this `impl` block are used to create a new model builder with a specific [`Element`] type.
impl ModelBuilder {
    /// Return output type
    pub fn kind(&self) -> OutputType {
        self.output
    }

    /// Create a new object from a body `{ ... }`.
    pub fn new_object_body() -> Self {
        Self {
            root: Object::default().into(),
            ..Default::default()
        }
    }

    /// Create a new 2D object.
    ///
    /// This function is used when a call to a sketch is evaluated.
    pub fn new_2d_object() -> Self {
        Self {
            root: Object::default().into(),
            output: OutputType::Geometry2D,
            ..Default::default()
        }
    }

    /// Create a new children placeholder
    pub fn new_children_placeholder() -> Self {
        Self {
            root: ModelInner::new(Refer::none(Element::ChildrenPlaceholder)),
            ..Default::default()
        }
    }

    /// Create a new 3D object.
    ///
    /// This function is used when a call to a part is evaluated.
    pub fn new_3d_object() -> Self {
        Self {
            root: Object::default().into(),
            output: OutputType::Geometry3D,
            ..Default::default()
        }
    }

    /// New 2D primitive.
    pub fn new_2d_primitive(geometry: std::rc::Rc<Geometry2D>) -> Self {
        Self {
            root: geometry.into(),
            output: OutputType::Geometry2D,
            ..Default::default()
        }
    }

    /// New 3D primitive.
    pub fn new_3d_primitive(geometry: std::rc::Rc<Geometry3D>) -> Self {
        Self {
            root: geometry.into(),
            output: OutputType::Geometry3D,
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
    pub fn new_operation<T: Operation + 'static>(operation: T, src_ref: SrcRef) -> Self {
        Self {
            root: ModelInner::new(Refer::new(Element::Operation(Rc::new(operation)), src_ref)),
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

    /// Add multiple children to the model if it matches.
    pub fn add_children2(&mut self, mut children: Models) -> EvalResult<()> {
        self.children.append(&mut children);
        Ok(())
    }

    /// Set object properties.
    pub fn properties(mut self, properties: ObjectProperties) -> Self {
        self.properties = properties;
        self
    }

    /// Set property value for object.
    pub fn set_property(&mut self, id: Identifier, value: Value) -> &mut Self {
        self.properties.insert(id, value);
        self
    }

    /// Return true if the object has a property with `id`.
    pub fn has_property(&self, id: &Identifier) -> bool {
        self.properties.contains_key(id)
    }

    /// Build a [`Model`].
    pub fn build(mut self) -> Model {
        if let Element::Object(object) = &mut self.root.element.value {
            object.props = self.properties
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

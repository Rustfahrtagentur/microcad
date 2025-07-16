// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node builder.

use microcad_core::{Geometry2D, Geometry3D};

use crate::{eval::*, model_tree::*, rc::*, src_ref::*, syntax::*, value::*};

/// A builder pattern to build model nodes
#[derive(Default)]
pub struct ModelNodeBuilder {
    root: ModelNodeInner,
    /// Properties to add to the model node if it is an [`Object`]
    pub properties: ObjectProperties,
    /// Children to add to this node.
    pub children: ModelNodes,

    /// The output type of this node.
    output: ModelNodeOutputType,
}

/// ModelNodeBuilder constructors.
///
/// All methods in this `impl` block are used to create a new model builder with a specific [`Element`] type.
impl ModelNodeBuilder {
    /// Return output type
    pub fn kind(&self) -> ModelNodeOutputType {
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
            output: ModelNodeOutputType::Geometry2D,
            ..Default::default()
        }
    }

    /// Create a new children placeholder
    pub fn new_children_placeholder() -> Self {
        Self {
            root: ModelNodeInner::new(Refer::none(Element::ChildrenPlaceholder)),
            ..Default::default()
        }
    }

    /// Create a new 3D object.
    ///
    /// This function is used when a call to a part is evaluated.
    pub fn new_3d_object() -> Self {
        Self {
            root: Object::default().into(),
            output: ModelNodeOutputType::Geometry3D,
            ..Default::default()
        }
    }

    /// New 2D primitive.
    pub fn new_2d_primitive(geometry: std::rc::Rc<Geometry2D>) -> Self {
        Self {
            root: geometry.into(),
            output: ModelNodeOutputType::Geometry2D,
            ..Default::default()
        }
    }

    /// New 3D primitive.
    pub fn new_3d_primitive(geometry: std::rc::Rc<Geometry3D>) -> Self {
        Self {
            root: geometry.into(),
            output: ModelNodeOutputType::Geometry3D,
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
            root: ModelNodeInner::new(Refer::new(Element::Operation(Rc::new(operation)), src_ref)),
            ..Default::default()
        }
    }
}

impl ModelNodeBuilder {
    /// Add multiple children to the node if it matches.
    pub fn add_children(mut self, mut children: ModelNodes) -> EvalResult<Self> {
        self.children.append(&mut children);
        Ok(self)
    }

    /// Add multiple children to the node if it matches.
    pub fn add_children2(&mut self, mut children: ModelNodes) -> EvalResult<()> {
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

    /// Build a [`ModelNode`].
    pub fn build(mut self) -> ModelNode {
        if let Element::Object(object) = &mut self.root.element.value {
            object.props = self.properties
        }

        let node = ModelNode::new(self.root.into());
        node.append_children(self.children);
        node.deduce_output_type();
        node
    }
}

impl std::fmt::Display for ModelNodeBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.properties)
    }
}

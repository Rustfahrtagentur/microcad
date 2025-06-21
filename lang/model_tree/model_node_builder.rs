// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node builder.

use std::rc::Rc;

use microcad_core::{Geometry2D, Geometry3D};

use crate::{eval::*, model_tree::*, src_ref::*, syntax::*, value::*};

/// A builder pattern to build model nodes
#[derive(Default)]
pub struct ModelNodeBuilder<'a> {
    inner: ModelNodeInner,
    /// Properties to add to the model node if it is an [`Object`]
    pub properties: ObjectProperties,
    /// Children to add to this node.
    pub children: ModelNodes,

    /// The output type of this node.
    output_type: ModelNodeOutputType,

    /// An optional context for error handling.
    context: Option<&'a mut Context>,
}

/// ModelNodeBuilder constructors.
///
/// All methods in this `impl` block are used to create a new model builder with a specific [`Element`] type.
impl<'a> ModelNodeBuilder<'a> {
    /// Create a new object from a body `{ ... }`.
    pub fn new_object_body() -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::none(Element::Object(Object::default()))),
            ..Default::default()
        }
    }

    /// Create a new 2D object.
    ///
    /// This function is used when a call to a sketch is evaluated.
    pub fn new_2d_object() -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::none(Element::Object(Object::default()))),
            output_type: ModelNodeOutputType::Geometry2D,
            ..Default::default()
        }
    }

    /// Create a new children placeholder
    pub fn new_children_placeholder() -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::none(Element::ChildrenPlaceholder)),
            ..Default::default()
        }
    }

    /// Create a new 3D object.
    ///
    /// This function is used when a call to a part is evaluated.
    pub fn new_3d_object() -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::none(Element::Object(Object::default()))),
            output_type: ModelNodeOutputType::Geometry3D,
            ..Default::default()
        }
    }

    /// New 2D primitive.
    pub fn new_2d_primitive(geometry: std::rc::Rc<Geometry2D>) -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::none(Element::Primitive2D(geometry))),
            output_type: ModelNodeOutputType::Geometry2D,
            ..Default::default()
        }
    }

    /// New 3D primitive.
    pub fn new_3d_primitive(geometry: std::rc::Rc<Geometry3D>) -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::none(Element::Primitive3D(geometry))),
            output_type: ModelNodeOutputType::Geometry3D,
            ..Default::default()
        }
    }

    /// New transform.
    pub fn new_transform(transform: AffineTransform) -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::none(Element::Transform(transform))),
            output_type: ModelNodeOutputType::NotDetermined,
            ..Default::default()
        }
    }

    /// New operation.
    pub fn new_operation<T: Operation + 'static>(operation: T, src_ref: SrcRef) -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::new(Element::Operation(Rc::new(operation)), src_ref)),
            output_type: ModelNodeOutputType::NotDetermined,
            ..Default::default()
        }
    }
}

impl<'a> ModelNodeBuilder<'a> {
    /// Determine the output type of this node by some child node.
    ///
    /// TODO: Replace `panic!` with context warnings.
    pub fn determine_output_type(&self, child: &ModelNode) -> EvalResult<ModelNodeOutputType> {
        match self.inner.element() {
            Element::ChildrenPlaceholder => panic!("A child placeholder cannot have children"),
            Element::Transform(_) => {
                if !self.inner.children().is_empty() {
                    panic!("A transformation cannot have more than one child.")
                }
            }
            Element::Operation(op) => {
                if !self.inner.children().is_empty() {
                    panic!("An operation cannot have more than one child.")
                } else {
                    return Ok(op.output_type(child));
                }
            }
            _ => {}
        }

        match child.output_type() {
            ModelNodeOutputType::NotDetermined => {
                return Ok(self.output_type.clone());
            }
            ModelNodeOutputType::Invalid => {
                panic!("Child node's output type is invalid.")
            }
            _ => {}
        }

        match self.output_type {
            ModelNodeOutputType::NotDetermined => {
                // Determine nodes output type by child output type.
            }
            ModelNodeOutputType::Geometry2D => {
                if child.output_type() == ModelNodeOutputType::Geometry3D {
                    panic!("Cannot nest a 2D geometry in a 3D geometry node.")
                }
            }
            ModelNodeOutputType::Geometry3D => {
                if child.output_type() == ModelNodeOutputType::Geometry2D {
                    panic!("Cannot nest a 3D geometry in a 2D geometry node.")
                }
            }
            ModelNodeOutputType::Invalid => {
                panic!("Invalid output type.")
            }
        }

        Ok(child.output_type())
    }

    /// Add multiple children to the node if it matches.
    pub fn add_children(mut self, children: ModelNodes) -> EvalResult<Self> {
        if let Some(child) = children.first() {
            //  TODO Check child's output type
            self.output_type = self.determine_output_type(child)?;
        }

        for child in children.iter() {
            self.children.push(child.clone());
        }

        Ok(self)
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
    pub fn has_property(&mut self, id: &Identifier) -> bool {
        self.properties.contains_key(id)
    }

    /// Build a [`ModelNode`].
    pub fn build(mut self) -> ModelNode {
        if let Element::Object(object) = self.inner.element_mut() {
            object.props = self.properties
        }
        self.inner.output_type = self.output_type;

        let node = ModelNode::new(self.inner);
        node.append_children(self.children)
    }
}

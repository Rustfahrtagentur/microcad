// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node builder.

use std::rc::Rc;

use microcad_core::{Geometry2D, Geometry3D};

use crate::{
    eval::{Context, EvalResult},
    model_tree::*,
    src_ref::{Refer, SrcRef},
};

pub struct ModelNodeBuilder {
    inner: ModelNodeInner,

    output_type: ModelNodeOutputType,
    context: Option<Context>,
}

/// ModelNodeBuilder constructors.
///
/// All methods in this `impl` block are used to create a new model builder with a specific [`Element`] type.
impl ModelNodeBuilder {
    /// Create a new object from a body `{ ... }`.
    fn new_object_body() -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::none(Element::Object(Object::default()))),
            output_type: ModelNodeOutputType::NotDetermined,
            context: None,
        }
    }

    /// Create a new 2D object.
    ///
    /// This function is used when a sketch is called.
    fn new_2d_object() -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::none(Element::Object(Object::default()))),
            output_type: ModelNodeOutputType::Geometry2D,
            context: None,
        }
    }

    /// Create a new 3D object.
    ///
    /// This function is used when a part is called.
    pub fn new_3d_object() -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::none(Element::Object(Object::default()))),
            output_type: ModelNodeOutputType::Geometry3D,
            context: None,
        }
    }

    /// New 2D primitive.
    pub fn new_2d_primitive(geometry: std::rc::Rc<Geometry2D>, src_ref: SrcRef) -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::new(Element::Primitive2D(geometry), src_ref)),
            output_type: ModelNodeOutputType::Geometry2D,
            context: None,
        }
    }

    /// New 3D primitive.
    pub fn new_3d_primitive(geometry: std::rc::Rc<Geometry3D>, src_ref: SrcRef) -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::new(Element::Primitive3D(geometry), src_ref)),
            output_type: ModelNodeOutputType::Geometry3D,
            context: None,
        }
    }

    /// New transform.
    pub fn new_transform(transform: AffineTransform, src_ref: SrcRef) -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::new(Element::Transform(transform), src_ref)),
            output_type: ModelNodeOutputType::NotDetermined,
            context: None,
        }
    }

    /// New operation.
    pub fn new_operation<T: Operation + 'static>(operation: T, src_ref: SrcRef) -> Self {
        Self {
            inner: ModelNodeInner::new(Refer::new(Element::Operation(Rc::new(operation)), src_ref)),
            output_type: ModelNodeOutputType::NotDetermined,
            context: None,
        }
    }
}

impl ModelNodeBuilder {
    /// Determine the output type of this node by some child node.
    ///
    /// TODO: Replace `panic!` with context warnings.
    pub fn determine_output_type(&self, child: &ModelNode) -> EvalResult<ModelNodeOutputType> {
        match child.output_type() {
            ModelNodeOutputType::NotDetermined => {
                panic!("Child node's output type must have been determined")
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
                if child.output_type() != self.output_type {
                    panic!("Cannot nest a 2D geometry in a 3D geometry node.")
                }
            }
            ModelNodeOutputType::Geometry3D => {
                if child.output_type() != self.output_type {
                    panic!("Cannot nest a 3D geometry in a 2D geometry node.")
                }
            }
            ModelNodeOutputType::Invalid => {
                panic!("Invalid output type.")
            }
        }

        match self.inner.element() {
            Element::ChildrenPlaceholder => panic!("A child placeholder cannot have children"),
            Element::Transform(_) => {
                if !self.inner.children().is_empty() {
                    panic!("A transformation cannot have more than one child.")
                }
            }
            Element::Operation(_) => {
                if !self.inner.children().is_empty() {
                    panic!("An operation cannot have more than one child.")
                }
            }
            _ => {}
        }

        Ok(child.output_type())
    }

    /// Add a new child to the node if it matches.
    ///
    /// Outputs a warning if the child node does not match and if a context is present.
    pub fn add_child(mut self, child: ModelNode) -> EvalResult<Self> {
        self.output_type = self.determine_output_type(&child)?;

        self.inner.add_child(child);
        Ok(self)
    }

    /// Add multiple children to the node if it matches.
    pub fn add_children(mut self, children: ModelNodes) -> EvalResult<Self> {
        if let Some(child) = children.first() {
            self.output_type = self.determine_output_type(child)?;
        }

        for child in children.iter() {
            self.inner.add_child(child.clone());
        }

        Ok(self)
    }

    pub fn add_metadata(mut self, metadata: Metadata) -> Self {
        self.inner.set_metadata(metadata);
        self
    }

    pub fn build_node(self) -> ModelNode {
        ModelNode::new(self.inner)
    }
}

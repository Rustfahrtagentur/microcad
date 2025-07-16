// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node

use crate::{model_tree::*, rc::*, resolve::*, src_ref::*, syntax::*};
use microcad_core::{Geometry2D, Geometry3D};

/// The actual node contents
#[derive(custom_debug::Debug, Default)]
pub struct ModelNodeInner {
    /// Optional id.
    ///
    /// The id is set when the model node was created by an assignment: `a = cube(50mm)`.
    pub id: Option<Identifier>,
    /// Parent object.
    #[debug(skip)]
    pub parent: Option<ModelNode>,
    /// Children of the model node.
    pub children: ModelNodes,
    /// Element of the node with [SrcRef].
    pub element: Refer<Element>,
    /// Attributes used for export.
    pub attributes: Attributes,
    /// The symbol (e.g. [`WorkbenchDefinition`]) that created this [`ModelNode`].
    pub origin: ModelNodeOrigin,
    /// The output type of the this node.
    pub output: ModelNodeOutput,
}

impl ModelNodeInner {
    /// Create a new [`ModelNodeInner`] with a specific element.
    pub fn new(element: Refer<Element>) -> Self {
        Self {
            element,
            ..Default::default()
        }
    }

    /// Clone only the content of this node without children and parent.
    pub fn clone_content(&self) -> Self {
        Self {
            id: self.id.clone(),
            parent: None,
            element: self.element.clone(),
            attributes: self.attributes.clone(),
            origin: self.origin.clone(),
            output: self.output.clone(),
            ..Default::default()
        }
    }

    /// Return iterator of children.s
    pub fn children(&self) -> std::slice::Iter<'_, ModelNode> {
        self.children.iter()
    }

    /// Return if node has no children.
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Set the information about the creator of this node.
    ///
    /// This function is called after the resulting nodes of a call of a part
    /// have been retrieved.   
    pub(crate) fn set_creator(&mut self, creator: Symbol, call_src_ref: SrcRef) {
        self.origin.creator = Some(creator);
        self.origin.call_src_ref = call_src_ref;
    }
}

impl From<Object> for ModelNodeInner {
    fn from(object: Object) -> Self {
        Self::new(Refer::none(Element::Object(object)))
    }
}

impl From<Rc<Geometry2D>> for ModelNodeInner {
    fn from(geometry: Rc<Geometry2D>) -> Self {
        Self::new(Refer::none(Element::Primitive2D(geometry)))
    }
}

impl From<Rc<Geometry3D>> for ModelNodeInner {
    fn from(geometry: Rc<Geometry3D>) -> Self {
        Self::new(Refer::none(Element::Primitive3D(geometry)))
    }
}

impl From<AffineTransform> for ModelNodeInner {
    fn from(transform: AffineTransform) -> Self {
        ModelNodeInner::new(Refer::none(Element::Transform(transform)))
    }
}

// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object builder module

use crate::{eval::*, model_tree::*, src_ref::*, syntax::*, value::*};

/// Object builder to build up a [ModelNode] with an object element.
#[derive(Default)]
pub struct ObjectBuilder {
    /// The object element to be built.
    object: Object,

    /// Children to be placed in the node.
    children: ModelNodes,

    /// The [`SrcRef`] attached to the object element.
    src_ref: SrcRef,
}

impl ObjectBuilder {
    /// Create new [ObjectBuilder] with a [SrcRef].
    pub fn new(src_ref: SrcRef) -> Self {
        Self {
            src_ref,
            ..Default::default()
        }
    }

    /// Append child nodes to this object node.
    pub fn append_children(&mut self, nodes: &mut ModelNodes) -> &mut Self {
        (*self.children).append(nodes);
        self
    }

    /// Set property value for object.
    pub fn set_property(&mut self, id: Identifier, value: Value) -> &mut Self {
        self.object.props.insert(id, value);
        self
    }

    /// Return true if the object has a property with `id`.
    pub fn has_property(&mut self, id: &Identifier) -> bool {
        self.object.props.contains_key(id)
    }

    /// Build the [`ModelNode`].
    pub fn build_node(self) -> ModelNode {
        let node = ModelNode::new_object(self.object, self.src_ref);
        node.append_children(self.children);
        node
    }
}

// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node

use crate::{modeltree::*, rc::*, resolve::Symbol, src_ref::*, syntax::*, value::*};
use microcad_core::*;

/// The actual node contents
#[derive(custom_debug::Debug, Clone, Default)]
pub struct ModelNodeInner {
    /// Optional id.
    ///
    /// The id is set when the model node was created by an assignment: `a = cube(50mm)`.
    id: Option<Identifier>,

    /// Parent object.
    #[debug(skip)]
    parent: Option<ModelNode>,

    // Children of the model node.
    children: Vec<ModelNode>,

    /// Element of the node with [SrcRef].
    element: Refer<Element>,

    /// Metadata.
    metadata: Metadata,

    // The symbol (e.g. [ModuleDefinition]) that created this object.
    symbol: Option<Symbol>,
}

impl ModelNodeInner {
    /// Return element of this node.
    pub fn element(&self) -> &Element {
        &self.element
    }

    /// Return reference to the metadata of this node.
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Set metadata for this node.
    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }
}

/// Children iterator struct.
pub struct Children {
    node: ModelNode,
    index: usize,
}

impl Children {
    /// Create new [`Children`] iterator
    pub fn new(node: ModelNode) -> Self {
        Self { node, index: 0 }
    }
}

impl Iterator for Children {
    type Item = ModelNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.0.borrow();
        let child = node.children.get(self.index);
        self.index += 1;
        child.cloned()
    }
}

/// Iterator over all descendants.
pub struct Descendants {
    stack: Vec<(ModelNode, usize)>,
}

impl Descendants {
    /// Create new descendants iterator
    pub fn new(root: ModelNode) -> Self {
        let mut stack = Vec::new();
        let children = &root.borrow().children;
        for child in children {
            stack.push((child.clone(), 0));
        }
        Self { stack }
    }
}

impl Iterator for Descendants {
    type Item = ModelNode;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((node, index)) = self.stack.pop() {
            let node_inner = node.0.borrow();
            let children = &node_inner.children;

            if index < children.len() {
                // Push the current node back with the next index
                self.stack.push((node.clone(), index + 1));

                // Push the current child onto the stack to process its children
                let child = &children[index];
                self.stack.push((child.clone(), 0));

                // Return the current child
                return Some(child.clone());
            }
        }
        None
    }
}

/// A reference counted, mutable [ObjectNode].
#[derive(Debug, Clone)]
pub struct ModelNode(RcMut<ModelNodeInner>);

impl ModelNode {
    /// Create new object node from element.
    pub fn new_element(element: Refer<Element>) -> Self {
        Self(RcMut::new(ModelNodeInner {
            element,
            ..Default::default()
        }))
    }

    /// Return a new empty object.
    pub fn new_empty_object(src_ref: SrcRef) -> Self {
        Self::new_element(Refer::new(Element::Object(Object::default()), src_ref))
    }

    /// Return a model node containing an [Object].
    pub fn new_object(object: Object, src_ref: SrcRef) -> Self {
        Self::new_element(Refer::new(Element::Object(object), src_ref))
    }

    /// Return an transformation node.
    pub fn new_transformation<T: Transformation + 'static>(
        transformation: T,
        src_ref: SrcRef,
    ) -> Self {
        Self::new_element(Refer::new(
            Element::Transformation(std::rc::Rc::new(transformation)),
            src_ref,
        ))
    }

    /// Return id of this object node.
    pub fn id(&self) -> Option<Identifier> {
        self.0.borrow().id.clone()
    }

    /// Set new id for this node.
    pub fn set_id(&mut self, id: Identifier) -> Self {
        self.0.borrow_mut().id = Some(id);
        self.clone()
    }

    /// Get borrowed reference to the inner of this node.
    pub fn borrow(&'_ self) -> std::cell::Ref<'_, ModelNodeInner> {
        self.0.borrow()
    }

    /// Calculate Depth of the node.
    pub fn depth(&self) -> usize {
        self.0
            .borrow()
            .parent
            .as_ref()
            .map_or(0, |parent| parent.depth() + 1)
    }

    /// Make a deep copy if this node.
    pub fn make_deep_copy(&self) -> Self {
        Self(RcMut::new(self.0.borrow().clone()))
    }

    /// Return address of this node.
    pub fn addr(&self) -> usize {
        self.0.as_ptr().addr()
    }

    /// Check if `other` is and `self` have the same address.
    pub fn is_same_as(&self, other: &ModelNode) -> bool {
        self.addr() == other.addr()
    }

    /// Remove child from this node.
    pub fn remove_child(&self, child: &ModelNode) {
        let mut s = self.0.borrow_mut();
        s.children.retain(|node| !node.is_same_as(child));
    }

    /// Detaches a node from its parent. Children are not affected.
    pub fn detach(&self) {
        match self.0.borrow_mut().parent {
            Some(ref mut parent) => {
                parent.remove_child(self);
            }
            None => return,
        }

        self.0.borrow_mut().parent = None;
    }

    /// Set parent of this node.
    pub fn set_parent(&mut self, parent: ModelNode) {
        self.0.borrow_mut().parent = Some(parent);
    }

    /// Return parent of this node.
    pub fn parent(&self) -> Option<ModelNode> {
        self.0.borrow().parent.clone()
    }

    /// Children iterator.
    pub fn children(&self) -> Children {
        Children::new(self.clone())
    }

    /// Returns an iterator of nodes to this node and its unnamed descendants, in tree order.
    ///
    /// Includes the current node.
    pub fn descendants(&self) -> Descendants {
        Descendants::new(self.clone())
    }

    /// Append a single node as child.
    pub fn append(&self, node: ModelNode) {
        let mut node = node.clone();
        node.set_parent(self.clone());
        self.0.borrow_mut().children.push(node);
    }

    /// Append a multiple nodes as children.
    ///
    /// Return self.
    pub fn append_children(&self, nodes: ModelNodes) -> Self {
        for node in nodes.iter() {
            self.append(node.clone())
        }
        self.clone()
    }

    /// Short cut to generate boolean operator as binary operation with two nodes.
    pub fn binary_op(self, op: BooleanOp, other: ModelNode) -> ModelNode {
        assert!(self != other, "lhs and rhs must be distinct.");
        ModelNode::new_transformation(op, SrcRef(None))
            .append_children(vec![self.clone(), other].into())
    }

    /// Find children node placeholder in node descendants
    pub fn find_children_placeholder(&self) -> Option<ModelNode> {
        self.descendants().find(|n| {
            n.id().is_some() && matches!(n.0.borrow().element.value, Element::ChildrenPlaceholder)
        })
    }

    /// Return inner node if we are in an [`Object`] node.
    pub fn into_inner(self) -> Option<ModelNode> {
        self.children().next().and_then(|n| {
            if let Element::Object(_) = n.0.borrow().element.value {
                Some(n.clone())
            } else {
                None
            }
        })
    }

    /// Dumps the tree structure of a node.
    ///
    /// The depth of a node is marked by the number of white spaces
    pub fn dump(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.descendants()
            .try_for_each(|child| writeln!(f, "{}{}", " ".repeat(child.depth()), child))
    }

    /// Set metadata.
    pub(crate) fn set_metadata(&self, metadata: Metadata) {
        self.0.borrow_mut().set_metadata(metadata);
    }

    /// Get value for name-value metadata with `id`.
    pub(crate) fn get_metadata_by_id(&self, id: &Identifier) -> Option<Value> {
        self.0
            .borrow()
            .metadata()
            .get_by_id(id)
            .map(|item| item.value())
    }

    /// Get a property from an object node.
    ///
    /// Only object nodes can have properties.
    pub fn get_property_value(&self, id: &Identifier) -> Option<Value> {
        self.borrow()
            .element()
            .get_property_value(id)
            .cloned()
            .or(self.get_metadata_by_id(id))
    }
}

impl PartialEq for ModelNode {
    fn eq(&self, other: &Self) -> bool {
        self.addr() == other.addr()
    }
}

impl std::fmt::Display for ModelNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,)
    }
}

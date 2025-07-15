// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node

mod model_node_builder;
mod model_node_inner;
mod model_nodes;
mod origin;

pub use model_node_builder::*;
pub use model_node_inner::*;
pub use model_nodes::*;
pub use origin::*;

use crate::{diag::WriteToFile, model_tree::*, rc::*, syntax::*, value::*, GetPropertyValue};
use derive_more::{Deref, DerefMut};
use microcad_core::*;

/// A reference counted, mutable [`ModelNode`].
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct ModelNode(RcMut<ModelNodeInner>);

impl ModelNode {
    /// Create new model node from inner.
    pub fn new(inner: RcMut<ModelNodeInner>) -> Self {
        Self(inner)
    }

    /// Calculate Depth of the node.
    pub fn depth(&self) -> usize {
        self.parents().count()
    }

    /// Make a deep copy if this node.
    /// TODO: isn't this a Clone?
    pub fn make_deep_copy(&self) -> Self {
        let copy = Self(RcMut::new(self.0.borrow().clone_content()));
        for child in self.borrow().children.iter() {
            copy.append(child.make_deep_copy());
        }
        copy
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

    /// Append a single node as child.
    ///
    /// Also tries to set the output type if it has not been determined yet.
    pub fn append(&self, node: ModelNode) -> ModelNode {
        node.borrow_mut().parent = Some(self.clone());

        let mut self_ = self.0.borrow_mut();
        self_.children.push(node.clone());

        node
    }

    /// Append multiple nodes as children.
    ///
    /// Return self.
    pub fn append_children(&self, nodes: ModelNodes) -> Self {
        for node in nodes.iter() {
            self.append(node.clone());
        }
        self.clone()
    }

    /// Short cut to generate boolean operator as binary operation with two nodes.
    pub fn boolean_op(self, op: BooleanOp, other: ModelNode) -> ModelNode {
        assert!(self != other, "lhs and rhs must be distinct.");
        ModelNodes::from(vec![self.clone(), other]).boolean_op(op)
    }

    /// Find children node placeholder in node descendants.
    pub fn find_children_placeholder(&self) -> Option<ModelNode> {
        self.descendants().find(|n| {
            n.borrow().id.is_none()
                && matches!(n.0.borrow().element.value, Element::ChildrenPlaceholder)
        })
    }

    /// Find the original source file of this node
    pub fn find_source_file(&self) -> Option<std::rc::Rc<SourceFile>> {
        self.ancestors()
            .find_map(|node| node.borrow().origin.source_file.clone())
    }

    /// Test if the node has this specific source file.
    pub fn has_source_file(&self, source_file: &std::rc::Rc<SourceFile>) -> bool {
        match (source_file.as_ref(), self.find_source_file()) {
            (a, Some(b)) => a.hash == b.hash,
            _ => false,
        }
    }

    /// Return inner node if we are in an [`Object`] node.
    pub fn into_inner_object_node(&self) -> Option<ModelNode> {
        self.borrow().children.iter().next().and_then(|n| {
            if let Element::Object(_) = n.0.borrow().element.value {
                Some(n.clone())
            } else {
                None
            }
        })
    }

    /// A [`ModelNode`] signature has the form "[id: ]ElementType[ = origin][ -> result_type]".
    pub fn signature(&self) -> String {
        let self_ = self.borrow();

        format!(
            "{id}{element_type}{origin} -> {output_type}",
            id = match &self_.id {
                Some(id) => format!("{id}: "),
                None => String::new(),
            },
            element_type = self_.element,
            origin = match self_.origin.creator {
                Some(_) => format!(" = {origin}", origin = self_.origin),
                None => String::new(),
            },
            output_type = self.final_output_type()
        )
    }
}

/// Iterator methods.
impl ModelNode {
    /// Returns an iterator of nodes to this node and its unnamed descendants, in tree order.
    ///
    /// Includes the current node.
    pub fn descendants(&self) -> Descendants {
        Descendants::new(self.clone())
    }

    /// Returns an iterator of nodes that belong to the same source file as this one
    pub fn source_file_descendants(&self) -> SourceFileDescendants {
        SourceFileDescendants::new(self.clone())
    }

    /// Parents iterator.
    pub fn parents(&self) -> Parents {
        Parents::new(self.clone())
    }

    /// Ancestors iterator.
    pub fn ancestors(&self) -> Ancestors {
        Ancestors::new(self.clone())
    }
}

impl GetPropertyValue for ModelNode {
    fn get_property_value(&self, id: &Identifier) -> Value {
        self.borrow().element.get_property_value(id)
    }
}

impl GetAttribute for ModelNode {
    fn get_attribute(&self, id: &Identifier) -> Option<crate::model_tree::Attribute> {
        self.borrow().attributes.get_attribute(id)
    }
}

impl PartialEq for ModelNode {
    fn eq(&self, other: &Self) -> bool {
        self.addr() == other.addr()
    }
}

/// Prints a [`ModelNode`].
///
/// A [`ModelNode`] signature has the form "[id: ]ElementType[ = origin][ -> result_type]".
/// The exemplary output will look like this:
///
/// ```custom
/// id: Object:
///     Object = std::geo2d::circle(radius = 3.0mm) -> Geometry2D:
///         Primitive = __builtin::geo2d::circle(radius = 3.0) -> Geometry2D`
/// ```
impl std::fmt::Display for ModelNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let depth = self.depth() * 2;
        writeln!(f, "{:depth$}{signature}", "", signature = self.signature())?;
        for child in self.borrow().children.iter() {
            write!(f, "{child}")?;
        }
        Ok(())
    }
}

impl WriteToFile for ModelNode {}

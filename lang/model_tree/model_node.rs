// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node

use crate::{
    diag::WriteToFile, model_tree::*, rc::*, resolve::*, src_ref::*, syntax::*, value::*,
    GetPropertyValue,
};

use microcad_core::*;

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

    /// Return iterator of children
    pub fn children(&self) -> std::slice::Iter<'_, ModelNode> {
        self.children.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}

/// A reference counted, mutable [`ModelNode`].
#[derive(Debug, Clone)]
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
        // If this node's output type has not been determined, try to get it from child node
        if self_.output.model_node_output_type() == ModelNodeOutputType::NotDetermined {
            self_.output = ModelNodeOutput::new(node.output_type());
        }
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
        let mut s = String::new();
        if let Some(id) = &self_.id {
            s += format!("{id}: ").as_str();
        }
        s += self_.element.to_string().as_str();
        if self_.origin.creator.is_some() {
            s += format!(" = {origin}", origin = self_.origin).as_str();
        }
        if !matches!(self.output_type(), ModelNodeOutputType::NotDetermined) {
            s += format!(" -> \"{output_type}\"", output_type = self.output_type()).as_str();
        }
        if !self_.is_empty() {
            s += ":";
        }
        s
    }
}

/// Implementation to store information about the node origin.
impl ModelNode {
    /// Set the information about the creator of this node.
    ///
    /// This function is called after the resulting nodes of a call of a part
    /// have been retrieved.   
    pub(crate) fn set_creator(&self, creator: Symbol, call_src_ref: SrcRef) {
        let origin_ = &mut self.borrow_mut().origin;
        origin_.creator = Some(creator);
        origin_.call_src_ref = call_src_ref;
    }

    /// Set the arguments with have been passed to this node.
    pub(crate) fn set_original_arguments(&self, arguments: Tuple) -> Self {
        self.borrow_mut().origin.arguments = arguments;
        self.clone()
    }

    /// Set the original source file this node has been created from.
    pub(crate) fn set_original_source_file(&self, source_file: Rc<SourceFile>) -> Self {
        self.borrow_mut().origin.source_file = Some(source_file);
        self.clone()
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

/// Model node attribute setter
impl ModelNode {
    /// Set attributes.
    pub fn set_attributes(&self, attributes: Attributes) -> Self {
        self.0.borrow_mut().attributes = attributes;
        self.clone()
    }
}

impl std::ops::Deref for ModelNode {
    type Target = RcMut<ModelNodeInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ModelNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

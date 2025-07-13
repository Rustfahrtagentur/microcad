// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node

use crate::{
    GetPropertyValue, diag::WriteToFile, model_tree::*, rc::*, resolve::*, src_ref::*, syntax::*,
    value::*,
};

use microcad_core::*;

/// The actual node contents
#[derive(custom_debug::Debug, Default)]
pub struct ModelNodeInner {
    /// Optional id.
    ///
    /// The id is set when the model node was created by an assignment: `a = cube(50mm)`.
    id: Option<Identifier>,
    /// Parent object.
    #[debug(skip)]
    parent: Option<ModelNode>,
    // Children of the model node.
    children: ModelNodes,
    /// Element of the node with [SrcRef].
    element: Refer<Element>,
    /// Attributes used for export.
    attributes: Attributes,
    /// The symbol (e.g. [`WorkbenchDefinition`]) that created this [`ModelNode`].
    origin: ModelNodeOrigin,
    /// The output type of the this node.
    output: ModelNodeOutput,
}

impl ModelNodeInner {
    /// Create a new [`ModelNodeInner`] with a specific element.
    pub fn new(element: Refer<Element>) -> Self {
        Self {
            element,
            ..Default::default()
        }
    }

    /// Return reference to the children of this node.
    pub fn children(&self) -> &ModelNodes {
        &self.children
    }

    /// Return element of this node.
    pub fn element(&self) -> &Element {
        &self.element
    }

    /// Return a mutable reference of the element of this node.
    pub fn element_mut(&mut self) -> &mut Element {
        &mut self.element
    }

    /// Return reference to the attributes of this node.
    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Return mutable reference for the attributes of this node
    pub fn attributes_mut(&mut self) -> &mut Attributes {
        &mut self.attributes
    }

    /// Set attribute for this node.
    pub fn set_attributes(&mut self, attributes: Attributes) {
        self.attributes = attributes;
    }

    /// Return reference to the attributes of this node.
    pub fn output(&self) -> &ModelNodeOutput {
        &self.output
    }

    /// Return mutable reference for the attributes of this node
    pub fn output_mut(&mut self) -> &mut ModelNodeOutput {
        &mut self.output
    }

    /// Set output for this node.
    pub fn set_output(&mut self, output: ModelNodeOutput) {
        self.output = output;
    }

    /// Return a reference to the model node origin.
    pub fn origin(&self) -> &ModelNodeOrigin {
        &self.origin
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
}

/// A reference counted, mutable [`ModelNode`].
#[derive(Debug, Clone)]
pub struct ModelNode(RcMut<ModelNodeInner>);

impl ModelNode {
    /// Create new model node from inner.
    pub fn new(inner: ModelNodeInner) -> Self {
        Self(RcMut::new(inner))
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

    /// Get mutable borrowed reference to the inner of this node.
    pub fn borrow_mut(&'_ self) -> std::cell::RefMut<'_, ModelNodeInner> {
        self.0.borrow_mut()
    }
    /// Calculate Depth of the node.
    pub fn depth(&self) -> usize {
        self.parents().count()
    }

    /// Make a deep copy if this node.
    pub fn make_deep_copy(&self) -> Self {
        let copy = Self(RcMut::new(self.0.borrow().clone_content()));
        for child in self.children() {
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

    /// Set parent of this node.
    pub fn set_parent(&mut self, parent: ModelNode) {
        self.0.borrow_mut().parent = Some(parent);
    }

    /// Append a single node as child.
    ///
    /// Also tries to set the output type if it has not been determined yet.
    pub fn append(&self, node: ModelNode) -> ModelNode {
        let mut node = node;
        node.set_parent(self.clone());

        let mut b = self.0.borrow_mut();
        // If this node's output type has not been determined, try to get it from child node
        if b.output.model_node_output_type() == ModelNodeOutputType::NotDetermined {
            b.set_output(ModelNodeOutput::new(node.output_type()));
        }
        b.children.push(node.clone());

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
    pub fn binary_op(self, op: BooleanOp, other: ModelNode) -> ModelNode {
        assert!(self != other, "lhs and rhs must be distinct.");

        let container = ModelNodeBuilder::new_object_body()
            .add_children(vec![self.clone(), other].into())
            .expect("No error")
            .build();

        ModelNodeBuilder::new_operation(op, SrcRef(None))
            .add_children(vec![container].into())
            .expect("No error")
            .build()
    }

    /// Find children node placeholder in node descendants.
    pub fn find_children_placeholder(&self) -> Option<ModelNode> {
        self.descendants().find(|n| {
            n.id().is_none() && matches!(n.0.borrow().element.value, Element::ChildrenPlaceholder)
        })
    }

    /// Find the original source file of this node
    pub fn find_source_file(&self) -> Option<std::rc::Rc<SourceFile>> {
        self.ancestors().find_map(|node| {
            let b = node.borrow();
            let origin = b.origin();
            origin.source_file.clone()
        })
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
        self.children().next().and_then(|n| {
            if let Element::Object(_) = n.0.borrow().element.value {
                Some(n.clone())
            } else {
                None
            }
        })
    }

    /// A [`ModelNode`] signature has the form "[id: ]ElementType[ = origin][ -> result_type]".
    pub fn signature(&self) -> String {
        let mut s = String::new();
        if let Some(id) = self.id() {
            s += format!("{id}: ").as_str();
        }
        s += self.borrow().element().to_string().as_str();
        if self.origin().creator.is_some() {
            s += format!(" = {origin}", origin = self.origin()).as_str();
        }
        if !matches!(self.output_type(), ModelNodeOutputType::NotDetermined) {
            s += format!(" -> \"{output_type}\"", output_type = self.output_type()).as_str();
        }
        if self.has_children() {
            s += ":";
        }
        s
    }
}

/// Implementation to store information about the node origin.
impl ModelNode {
    /// Return the [`ModelNodeOrigin`] that created this node.
    pub fn origin(&self) -> ModelNodeOrigin {
        self.borrow().origin.clone()
    }

    /// Set the information about the creator of this node.
    ///
    /// This function is called after the resulting nodes of a call of a part
    /// have been retrieved.   
    pub(crate) fn set_creator(&self, creator: Symbol, call_src_ref: SrcRef) {
        let origin = &mut self.0.borrow_mut().origin;
        origin.creator = Some(creator);
        origin.call_src_ref = call_src_ref;
    }

    /// Set the arguments with have been passed to this node.
    pub(crate) fn set_original_arguments(&self, arguments: Tuple) -> Self {
        let origin = &mut self.0.borrow_mut().origin;
        origin.arguments = arguments;
        self.clone()
    }

    /// Set the original source file this node has been created from.
    pub(crate) fn set_original_source_file(&self, source_file: Rc<SourceFile>) -> Self {
        let origin = &mut self.0.borrow_mut().origin;
        origin.source_file = Some(source_file);
        self.clone()
    }
}

/// Iterator methods.
impl ModelNode {
    /// Return parent of this node.
    pub fn parent(&self) -> Option<ModelNode> {
        self.0.borrow().parent.clone()
    }

    /// Returns `true` if this node has children.
    pub fn has_children(&self) -> bool {
        !self.borrow().children().is_empty()
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
        self.0.borrow_mut().set_attributes(attributes);
        self.clone()
    }
}

impl GetPropertyValue for ModelNode {
    fn get_property_value(&self, id: &Identifier) -> Value {
        self.borrow().element().get_property_value(id)
    }
}

impl GetAttribute for ModelNode {
    fn get_attribute(&self, id: &Identifier) -> Option<crate::model_tree::Attribute> {
        self.borrow().attributes().get_attribute(id)
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
/// The examplary output will look like this:
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
        for child in self.children() {
            write!(f, "{child}")?;
        }
        Ok(())
    }
}

impl WriteToFile for ModelNode {}

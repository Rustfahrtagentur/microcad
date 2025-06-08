// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node

use crate::{
    eval::ArgumentMap,
    model_tree::*,
    rc::*,
    resolve::{FullyQualify, Symbol},
    src_ref::*,
    syntax::*,
    value::*,
};
use microcad_core::*;
use strum::IntoStaticStr;

/// The origin is the [`Symbol`] and [`ArgumentMap`] from which the node has been created.
#[derive(Clone, Debug)]
pub struct ModelNodeOrigin {
    /// The original symbol that has been called.
    symbol: Symbol,

    /// The original call arguments.
    arguments: ArgumentMap,
}

impl std::fmt::Display for ModelNodeOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{symbol}({arguments})",
            symbol = self.symbol.full_name(),
            arguments = self.arguments.to_oneline_string(Some(32))
        )
    }
}

/// The output type of the [`ModelNode`].
#[derive(Debug, Clone, IntoStaticStr, Default)]
pub enum ModelNodeOutputType {
    /// The output type has not yet been determined.
    #[default]
    NotDetermined,

    /// The [`ModelNode`] outputs a 2d geometry.
    Geometry2D,

    /// The [`ModelNode`] outputs a 3d geometry.
    Geometry3D,

    /// The [`ModelNode`] is invalid, you cannot mix 2d and 3d geometry.
    Invalid,
}

impl std::fmt::Display for ModelNodeOutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &'static str = self.into();
        write!(f, "{name}")
    }
}

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

    /// Metadata.
    metadata: Metadata,

    /// The symbol (e.g. [`PartDefinition`]) that created this [`ModelNode`].
    origin: Option<ModelNodeOrigin>,

    /// The output type of the this node.
    output_type: ModelNodeOutputType,
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

    /// Return reference to the children of this node.
    pub fn children(&self) -> &ModelNodes {
        &self.children
    }

    /// Set metadata for this node.
    pub fn set_metadata(&mut self, metadata: Metadata) {
        self.metadata = metadata;
    }

    /// Clone only the content of this node without children and parent.
    pub fn clone_content(&self) -> Self {
        Self {
            id: self.id.clone(),
            parent: None,
            children: ModelNodes::default(),
            element: self.element.clone(),
            metadata: self.metadata.clone(),
            origin: self.origin.clone(),
            output_type: self.output_type.clone(),
        }
    }
}

/// A reference counted, mutable [`ModelNode`].
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
    pub fn new_transformation<T: Transformer + 'static>(
        transformation: T,
        src_ref: SrcRef,
    ) -> Self {
        Self::new_element(Refer::new(
            Element::Transformer(std::rc::Rc::new(transformation)),
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

    /// Return the [`ModelNodeOrigin`] that created this node.
    pub fn origin(&self) -> Option<ModelNodeOrigin> {
        self.borrow().origin.clone()
    }

    /// Return output type.
    pub fn output_type(&self) -> ModelNodeOutputType {
        self.borrow().output_type.clone()
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

    /// Append a single node as child.
    pub fn append(&self, node: ModelNode) -> ModelNode {
        let mut node = node;
        node.set_parent(self.clone());
        self.0.borrow_mut().children.push(node.clone());
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
        ModelNode::new_transformation(op, SrcRef(None))
            .append_children(vec![self.clone(), other].into())
    }

    /// Find children node placeholder in node descendants
    pub fn find_children_placeholder(&self) -> Option<ModelNode> {
        self.descendants().find(|n| {
            n.id().is_none() && matches!(n.0.borrow().element.value, Element::ChildrenPlaceholder)
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

    /// A [`ModelNode`] signature has the form "[id: ]ElementType[ = origin][ -> result_type]".
    pub fn signature(&self) -> String {
        let mut s = String::new();
        if let Some(id) = self.id() {
            s += format!("{id}: ").as_str();
        }
        s += self.borrow().element().to_string().as_str();
        if let Some(origin) = self.origin() {
            s += format!(" = \"{origin}\"").as_str();
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

impl PartialEq for ModelNode {
    fn eq(&self, other: &Self) -> bool {
        self.addr() == other.addr()
    }
}

/// Prints a [`ModelNode`].
///
/// A [`ModelNode`] signature has the form "[id: ]ElementType[ = origin][ -> result_type]".
/// The examplary output will look like this:
/// ```
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

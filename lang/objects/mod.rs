// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object tree module

pub mod algorithm;
pub mod object;
pub mod object_attributes;
pub mod object_builder;
pub mod object_properties;

pub use algorithm::*;
pub use object::*;
pub use object_attributes::*;
pub use object_builder::*;
pub use object_properties::*;

use crate::{rc::*, resolve::Symbol, src_ref::SrcRef, syntax::*, value::*};
use microcad_core::*;
use strum::IntoStaticStr;

/// Inner of a node
#[derive(Clone, IntoStaticStr, Debug)]
pub enum ObjectNodeContent {
    /// An object that contains children and holds properties
    Object(Object),

    /// A special node after which children will be nested as siblings
    ChildrenNodeMarker,

    /// Generated 2D geometry.
    Primitive2D(Rc<Primitive2D>),

    /// Generated 3D geometry.
    #[cfg(feature = "geo3d")]
    Primitive3D(Rc<Primitive3D>),

    /// An algorithm trait that manipulates the node or its children
    Algorithm(Rc<dyn Algorithm>),
}

impl ObjectNodeContent {
    /// Get a property from an object node.
    ///
    /// Only object nodes can have properties.
    pub fn get_property_value(&self, id: &Identifier) -> Option<Value> {
        match self {
            Self::Object(object) => object
                .get_property_value(id)
                .cloned()
                .or(object.get_attribute_value(id)),
            _ => None,
        }
    }

    /// Assign object attributes.
    pub fn assign_object_attributes(&mut self, attributes: &mut ObjectAttributes) {
        if let Self::Object(object) = self {
            object.assign_object_attributes(attributes)
        }
    }
}

/// The default [`ObjectNodeContent`] is an empty [`Object`].
impl Default for ObjectNodeContent {
    fn default() -> Self {
        ObjectNodeContent::Object(Object::default())
    }
}

impl std::fmt::Display for ObjectNodeContent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name: &'static str = self.into();
        write!(f, "{name}")?;

        match &self {
            ObjectNodeContent::Algorithm(algorithm) => {
                write!(f, "({algorithm:?})")
            }
            ObjectNodeContent::Primitive2D(primitive2d) => {
                write!(f, "({primitive2d:?})")
            }
            ObjectNodeContent::Primitive3D(primitive3d) => {
                write!(f, "({primitive3d:?})")
            }
            _ => Ok(()),
        }
    }
}

/// The actual node contents
#[derive(custom_debug::Debug, Clone, Default)]
pub struct ObjectNodeInner {
    /// Optional id.
    ///
    /// The id is set when the object was created by an assignment: `a = cube(50mm)`.
    id: Option<Identifier>,

    /// Parent object.
    #[debug(skip)]
    parent: Option<ObjectNode>,

    // Children created by expression statements `cube(50mm);`.
    children: Vec<ObjectNode>,

    // Actual content of the node [Primitive], [Algorithm] etc
    content: ObjectNodeContent,

    // Optional SrcRef from where the object has been created
    src_ref: SrcRef,

    // The symbol (e.g. [ModuleDefinition]) that created this object.
    symbol: Option<Symbol>,
    // Hash of the node, 0 by default
    //hash: u64,
    //matrix: Mat4,
    //precision: f64,
}

impl ObjectNodeInner {
    /// Return content of this node.
    pub fn content(&self) -> &ObjectNodeContent {
        &self.content
    }
}

/// Children iterator struct.
pub struct Children {
    node: ObjectNode,
    index: usize,
}

impl Children {
    /// Create new [`Children`] iterator
    pub fn new(node: ObjectNode) -> Self {
        Self { node, index: 0 }
    }
}

impl Iterator for Children {
    type Item = ObjectNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node.0.borrow();
        let child = node.children.get(self.index);
        self.index += 1;
        child.cloned()
    }
}

/// Iterator over all descendants.
pub struct Descendants {
    stack: Vec<(ObjectNode, usize)>,
}

impl Descendants {
    /// Create new descendants iterator
    pub fn new(root: ObjectNode) -> Self {
        let mut stack = Vec::new();
        let children = &root.borrow().children;
        for child in children {
            stack.push((child.clone(), 0));
        }
        Self { stack }
    }
}

impl Iterator for Descendants {
    type Item = ObjectNode;

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
pub struct ObjectNode(RcMut<ObjectNodeInner>);

impl ObjectNode {
    /// Create new object node from content.
    pub fn new_from_content(content: ObjectNodeContent) -> Self {
        Self(RcMut::new(ObjectNodeInner {
            content,
            ..Default::default()
        }))
    }

    /// Return a new empty object.
    pub fn new_empty_object() -> Self {
        Self(RcMut::new(ObjectNodeInner::default()))
    }

    /// Return an object node containing an [Object].
    pub fn new_object(object: Object) -> Self {
        Self::new_from_content(ObjectNodeContent::Object(object))
    }

    /// Return an algorithm node.
    pub fn new_algorithm<T: Algorithm + 'static>(algorithm: T) -> Self {
        Self::new_from_content(ObjectNodeContent::Algorithm(std::rc::Rc::new(algorithm)))
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
    pub fn borrow(&'_ self) -> std::cell::Ref<'_, ObjectNodeInner> {
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
    pub fn is_same_as(&self, other: &ObjectNode) -> bool {
        self.addr() == other.addr()
    }

    /// Remove child from this node.
    pub fn remove_child(&self, child: &ObjectNode) {
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
    pub fn set_parent(&mut self, parent: ObjectNode) {
        self.0.borrow_mut().parent = Some(parent);
    }

    /// Return parent of this node.
    pub fn parent(&self) -> Option<ObjectNode> {
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
    pub fn append(&self, node: ObjectNode) {
        let mut node = node.clone();
        node.set_parent(self.clone());
        self.0.borrow_mut().children.push(node);
    }

    /// Append a multiple nodes as children.
    ///
    /// Return self.
    pub fn append_children(&self, nodes: ObjectNodes) -> Self {
        for node in nodes.iter() {
            self.append(node.clone())
        }
        self.clone()
    }

    /// Short cut to generate boolean operator as binary operation with two nodes.
    pub fn binary_op(self, op: BooleanOp, other: ObjectNode) -> ObjectNode {
        assert!(self != other, "lhs and rhs must be distinct.");
        ObjectNode::new_algorithm(op).append_children(vec![self.clone(), other].into())
    }

    /// Find children node marker in node descendants
    fn find_children_marker(&self) -> Option<ObjectNode> {
        self.descendants().find(|n| {
            n.id().is_some()
                && matches!(n.0.borrow().content, ObjectNodeContent::ChildrenNodeMarker)
        })
    }

    /// Return inner ObjectNode if we are in an object node.
    pub fn into_inner_object(self) -> Option<ObjectNode> {
        self.children().next().and_then(|n| {
            if let ObjectNodeContent::Object(_) = n.0.borrow().content {
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

    /// Assign object attributes.
    pub(crate) fn assign_object_attributes(&self, attributes: &mut ObjectAttributes) {
        self.0
            .borrow_mut()
            .content
            .assign_object_attributes(attributes);
    }
}

impl PartialEq for ObjectNode {
    fn eq(&self, other: &Self) -> bool {
        self.addr() == other.addr()
    }
}

impl std::fmt::Display for ObjectNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,)
    }
}

/// Object node multiplicities.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ObjectNodes(Vec<ObjectNode>);

impl ObjectNodes {
    /// Returns the first node if there is exactly one node in the list.
    pub fn single_node(&self) -> Option<ObjectNode> {
        match self.0.len() {
            1 => self.0.first().cloned(),
            _ => None,
        }
    }

    /// Nest a Vec of node multiplicities
    ///
    /// * `node_stack`: A list of node lists.
    ///
    /// The reference to the first stack element will be returned.
    ///
    /// Assume, our node stack `Vec<Vec<Node>>` has for lists `a`, `b`, `c`, `d`:
    /// ```
    /// let nodes = vec![
    ///     vec![obj("a0"), obj("a1")],
    ///     vec![obj("b0")],
    ///     vec![obj("c0"), obj("c1"), obj("c2")],
    ///     vec![obj("d0")],
    /// ];
    /// ```
    ///
    /// This should result in following node multiplicity:
    /// a0
    ///   b0
    ///     c0
    ///       d0
    ///     c1
    ///       d0
    ///     c2
    ///       d0
    /// a1
    ///   b0
    ///     c0
    ///       d0
    ///     c1
    ///       d0
    ///     c2
    ///       d0
    pub fn from_node_stack(node_stack: &[ObjectNodes]) -> Self {
        match node_stack.len() {
            0 => panic!("Node stack must not be empty"),
            1 => {}
            n => {
                (1..n)
                    .rev()
                    .map(|i| (&node_stack[i], &node_stack[i - 1]))
                    .for_each(|(prev_list, next_list)| {
                        // Insert a copy of each element `node` from `prev_list` as child to each element `new_parent` in `next_list`
                        next_list.iter().for_each(|new_parent_node| {
                            prev_list.iter().for_each(|node| {
                                node.detach();

                                // Handle children marker.
                                // If we have found a children marker node, use it's parent as new parent node.
                                let new_parent_node = match new_parent_node.find_children_marker() {
                                    Some(children_marker) => {
                                        let parent =
                                            children_marker.parent().expect("Must have a parent");
                                        children_marker.detach(); // Remove children marker from tree
                                        parent
                                    }
                                    None => new_parent_node.clone(),
                                };

                                new_parent_node.append(node.make_deep_copy());
                            });
                        });
                    });
            }
        }

        node_stack[0].clone()
    }

    /// Return an algorithm node that unites all children.
    pub fn union(&self) -> ObjectNode {
        match self.single_node() {
            Some(node) => node,
            None => {
                let union_node = ObjectNode::new_algorithm(BooleanOp::Union);
                union_node.append_children(self.clone())
            }
        }
    }

    /// Merge two lists of [`ObjectNode`] into one by concatenation.
    pub fn merge(lhs: ObjectNodes, rhs: ObjectNodes) -> Self {
        lhs.iter()
            .chain(rhs.iter())
            .cloned()
            .collect::<Vec<_>>()
            .into()
    }

    /// Dump all nodes.
    pub fn dump(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for node in self.iter() {
            node.dump(f)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for ObjectNodes {
    type Target = Vec<ObjectNode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ObjectNodes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<ObjectNode>> for ObjectNodes {
    fn from(value: Vec<ObjectNode>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for ObjectNodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in self.iter() {
            node.fmt(f)?;
        }
        Ok(())
    }
}

#[test]
fn node_nest() {
    fn obj(id: &str) -> ObjectNode {
        ObjectNode::new_empty_object().set_id(Identifier::no_ref(id))
    }

    let nodes = vec![
        vec![obj("a0"), obj("a1")].into(),
        vec![obj("b0")].into(),
        vec![obj("c0"), obj("c1"), obj("c2")].into(),
        vec![obj("d0")].into(),
    ];

    // This should result in following node multiplicity:
    // a0
    //   b0
    //     c0
    //       d0
    //     c1
    //       d0
    //     c2
    //       d0
    // a1
    //   b0
    //     c0
    //       d0
    //     c1
    //       d0
    //     c2
    //       d0
    let nodes = ObjectNodes::from_node_stack(&nodes);
    assert_eq!(nodes.len(), 2); // Contains a0 and a1 as root

    for node in nodes.iter() {
        node.descendants().for_each(|n| {
            log::trace!(
                "{}{}",
                "  ".repeat(n.depth()),
                match n.0.borrow().content() {
                    ObjectNodeContent::Object(_) => node.id().expect("Id").clone(),
                    _ => panic!("Object with name expected"),
                }
            )
        });
    }
}

// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object tree module

pub mod algorithm;
pub mod boolean_op;
pub mod object;
pub mod object_attributes;
pub mod object_builder;
pub mod object_properties;
pub mod transform;

pub use algorithm::*;
pub use object::*;
pub use object_attributes::*;
pub use object_builder::*;
pub use object_properties::*;
pub use transform::*;

use crate::{rc::*, syntax::*, value::*};
use microcad_core::*;
use strum::IntoStaticStr;

/// Inner of a node
#[derive(Clone, IntoStaticStr, Debug)]
pub enum ObjectNodeInner {
    /// An object that contains children and holds properties
    Object(Object),

    /// A special node after which children will be nested as siblings
    ChildrenNodeMarker,

    /// A generated 2D geometry
    Primitive2D(Rc<Primitive2D>),

    /// Generated 3D geometry
    #[cfg(feature = "geo3d")]
    Primitive3D(Rc<Primitive3D>),

    /// An algorithm trait that manipulates the node or its children
    Algorithm(Rc<dyn Algorithm>),

    /// An affine transformation of a geometry
    Transform(Transform),
}

impl ObjectNodeInner {
    /// Get a property from an object node
    ///
    /// Only object nodes can have properties.
    pub fn get_property_value(&self, id: &Identifier) -> Option<&Value> {
        match self {
            Self::Object(object) => object.get_property_value(id),
            _ => None,
        }
    }
}

impl std::fmt::Display for ObjectNodeInner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name: &'static str = self.into();
        write!(f, "{name}")?;

        match &self {
            ObjectNodeInner::Algorithm(algorithm) => {
                write!(f, "({algorithm:?})")
            }
            ObjectNodeInner::Primitive2D(primitive2d) => {
                write!(f, "({primitive2d:?})")
            }
            ObjectNodeInner::Primitive3D(primitive3d) => {
                write!(f, "({primitive3d:?})")
            }
            ObjectNodeInner::Transform(transform) => {
                write!(f, "({transform:?})")
            }
            _ => Ok(()),
        }
    }
}

/// Render node
pub type ObjectNode = rctree::Node<ObjectNodeInner>;

/// Create new object node with properties
pub fn object(object: Object) -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Object(object))
}

/// Create new object node without properties
pub fn empty_object() -> ObjectNode {
    object(Object::default())
}

/// Create a new transform node
pub fn transform(transform: Transform) -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Transform(transform))
}

/// Trait to calculate depth for a node
pub trait Depth {
    /// Calculate depth
    fn depth(&self) -> usize;
}

impl Depth for ObjectNode {
    fn depth(&self) -> usize {
        self.parent().map_or(0, |parent| parent.depth() + 1)
    }
}

/// Find children node marker in node descendants
fn find_children_marker(node: &ObjectNode) -> Option<ObjectNode> {
    node.descendants()
        .find(|n| matches!(*n.borrow(), ObjectNodeInner::ChildrenNodeMarker))
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
pub fn nest_nodes(node_stack: &[Vec<ObjectNode>]) -> &Vec<ObjectNode> {
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
                            let new_parent_node = match find_children_marker(new_parent_node) {
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

    &node_stack[0]
}

/// Dumps the tree structure of a node.
///
/// The depth of a node is marked by the number of white spaces
pub fn dump(f: &mut std::fmt::Formatter, node: ObjectNode) -> std::fmt::Result {
    use Depth;
    node.descendants()
        .try_for_each(|child| writeln!(f, "{}{}", " ".repeat(child.depth()), child.borrow()))
}

/// Return inner ObjectNode if we are in an object node
pub fn into_inner_object(node: ObjectNode) -> Option<ObjectNode> {
    node.first_child().and_then(|n| {
        if let ObjectNodeInner::Object(_) = *n.borrow() {
            Some(n.clone())
        } else {
            None
        }
    })
}

#[test]
fn node_nest() {
    fn obj(name: &str) -> ObjectNode {
        object(Object {
            id: name.into(),
            ..Default::default()
        })
    }

    let nodes = vec![
        vec![obj("a0"), obj("a1")],
        vec![obj("b0")],
        vec![obj("c0"), obj("c1"), obj("c2")],
        vec![obj("d0")],
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

    let nodes = nest_nodes(&nodes);
    assert_eq!(nodes.len(), 2); // Contains a0 and a1 as root

    for node in nodes {
        node.descendants().for_each(|n| {
            log::trace!(
                "{}{}",
                "  ".repeat(n.depth()),
                match *n.borrow() {
                    ObjectNodeInner::Object(ref obj) => obj.id.clone(),
                    _ => panic!("Object with name expected"),
                }
            )
        });
    }
}

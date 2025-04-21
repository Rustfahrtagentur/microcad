// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object tree

pub mod algorithm;
pub mod boolean_op;
pub mod object;
pub mod transform;

pub use algorithm::*;
pub use transform::*;
pub use object::*;

use crate::{rc_mut::*, value::Value, Id};
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

    /// An export node that exports the geometry to a file
    Export(ExportSettings),
}

impl ObjectNodeInner {
    /// Get a property from an object node
    ///
    /// Only Group nodes can have properties.
    pub fn get_property_value(&self, id: &Id) -> Option<&Value> {
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

/// Create new group node with properties
pub fn object(object: Object) -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Object(object))
}

/// Create new group node without properties
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
    node.descendants().find(|n| matches!(*n.borrow(), ObjectNodeInner::ChildrenNodeMarker))
}

/// Nest a Vec of node multiplicities
///
/// Assume, our node stack `Vec<Vec<Node>>` has for groups `a`, `b`, `c`, `d`:
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
pub fn nest_nodes(node_stack: Vec<Vec<ObjectNode>>) -> Vec<ObjectNode> {
    assert!(!node_stack.is_empty());
    
    if node_stack.len() >= 2 {
        let mut index = node_stack.len() - 1;

        loop {
            let next_group = node_stack.get(index).expect("Group expected");
            index -= 1;
            let group = node_stack.get(index).expect("Node group expected");
            for root in group.iter() {
                for node in next_group.iter() {
                    node.detach();
                    
                    let new_parent_node = match find_children_marker(root)  {
                        Some(children_marker) => {
                            let parent = children_marker.parent().expect("Must have a parent");
                            children_marker.detach(); // Remove children marker
                            parent
                        }
                        None => {
                            root.clone()
                        }
                    };

                    new_parent_node.append(node.make_deep_copy());
                }
            }

            if index == 0 {
                return group.clone();
            }
        }
    } 

    node_stack[0].clone()
}

/// Dumps the tree structure of a node.
///
/// The depth of a node is marked by the number of white spaces
pub fn dump(f: &mut std::fmt::Formatter, node: ObjectNode) -> std::fmt::Result {
    use Depth;
    node.descendants()
        .try_for_each(|child| writeln!(f, "{}{}", " ".repeat(child.depth()), child.borrow()))
}

/// Return ObjectNode if we are in a Group
pub fn into_group(node: ObjectNode) -> Option<ObjectNode> {
    node.first_child().and_then(|n| {
        if let ObjectNodeInner::Object(_) = *n.borrow() {
            Some(n.clone())
        } else {
            None
        }
    })
}

/// This function bakes the object node tree into a 2D geometry tree
pub fn bake2d(
    renderer: &mut Renderer2D,
    node: ObjectNode,
) -> core::result::Result<geo2d::Node, CoreError> {
    let node2d = {
        match *node.borrow() {
            ObjectNodeInner::Object(_) => geo2d::tree::group(),
            ObjectNodeInner::Export(_) => geo2d::tree::group(),
            ObjectNodeInner::Primitive2D(ref renderable) => {
                return Ok(geo2d::tree::geometry(
                    renderable.request_geometry(renderer)?,
                ));
            }
            ObjectNodeInner::Algorithm(ref algorithm) => {
                return algorithm.process_2d(
                    renderer,
                    crate::objects::into_group(node.clone()).unwrap_or(node.clone()),
                );
            }
            ObjectNodeInner::Transform(ref transform) => transform.into(),
            ObjectNodeInner::ChildrenNodeMarker => geo2d::tree::group(),
            _ => return Err(CoreError::NotImplemented),
        }
    };

    node.children().try_for_each(|child| {
        if let Ok(child) = bake2d(renderer, child) {
            node2d.append(child);
            Ok(())
        } else {
            Err(CoreError::NotImplemented)
        }
    })?;

    Ok(node2d)
}

/// This function bakes the object node tree into a 3D geometry tree
pub fn bake3d(
    renderer: &mut Renderer3D,
    node: ObjectNode,
) -> core::result::Result<geo3d::Node, CoreError> {
    let node3d = {
        match *node.borrow() {
            ObjectNodeInner::Object(_) => geo3d::tree::group(),
            ObjectNodeInner::Export(_) => geo3d::tree::group(),
            ObjectNodeInner::Primitive3D(ref renderable) => {
                return Ok(geo3d::tree::geometry(
                    renderable.request_geometry(renderer)?,
                ));
            }
            ObjectNodeInner::Algorithm(ref algorithm) => {
                return algorithm.process_3d(
                    renderer,
                    crate::objects::into_group(node.clone()).unwrap_or(node.clone()),
                );
            }
            ObjectNodeInner::Transform(ref transform) => transform.into(),
            ObjectNodeInner::ChildrenNodeMarker => geo3d::tree::group(),
            _ => return Err(CoreError::NotImplemented),
        }
    };

    node.children().try_for_each(|child| {
        if let Ok(child) = bake3d(renderer, child) {
            node3d.append(child);
            Ok(())
        } else {
            Err(CoreError::NotImplemented)
        }
    })?;

    Ok(node3d)
}

#[test]
fn node_nest() {
    fn obj(name: &str) -> ObjectNode {
        object(Object{name: name.into(), ..Default::default()})
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


    let nodes = nest_nodes(nodes.clone());
    assert_eq!(nodes.len(), 2); // Contains a0 and a1 as root

    for node in nodes {
        node
            .descendants()
            .for_each(|n| println!("{}{}", "  ".repeat(n.depth()), match *n.borrow() {
                ObjectNodeInner::Object(ref obj) => obj.name.clone(),
                _ => panic!("Object with name expected")
            }));
    }
}

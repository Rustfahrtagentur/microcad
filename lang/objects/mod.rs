// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render tree

pub mod algorithm;
pub mod boolean_op;
pub mod transform;

pub use algorithm::*;
pub use transform::*;

use strum::IntoStaticStr;

use crate::sym::*;
use microcad_core::*;

/// Inner of a node
#[derive(Clone, IntoStaticStr)]
pub enum ObjectNodeInner {
    /// A group node that contains children
    Group(SymbolTable),

    /// A special node after which children will be nested as siblings
    ChildrenNodeMarker,

    /// A generated 2D geometry
    Primitive2D(std::rc::Rc<Primitive2D>),

    /// Generated 3D geometry
    #[cfg(feature = "geo3d")]
    Primitive3D(std::rc::Rc<Primitive3D>),

    /// An algorithm trait that manipulates the node or its children
    Algorithm(std::rc::Rc<dyn Algorithm>),

    /// An affine transformation of a geometry
    Transform(Transform),

    /// An export node that exports the geometry to a file
    Export(ExportSettings),
}

impl std::fmt::Debug for ObjectNodeInner {
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

impl Symbols for ObjectNode {
    fn fetch(&self, id: &Id) -> Option<std::rc::Rc<Symbol>> {
        match *self.borrow() {
            ObjectNodeInner::Group(ref table) => table.fetch(id),
            _ => unreachable!(),
        }
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        match *self.borrow_mut() {
            ObjectNodeInner::Group(ref mut table) => table.add(symbol),
            _ => unreachable!(),
        };
        self
    }

    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self {
        match *self.borrow_mut() {
            ObjectNodeInner::Group(ref mut table) => table.add_alias(symbol, alias),
            _ => unreachable!(),
        };
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) -> SymResult<()> {
        match *self.borrow_mut() {
            ObjectNodeInner::Group(ref mut table) => table.copy(into),
            _ => unreachable!(),
        }
    }
}

/// Create new group node
pub fn group() -> ObjectNode {
    ObjectNode::new(ObjectNodeInner::Group(SymbolTable::default()))
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

/// Nest a Vec of nodes
///
/// Assume, our `Vec<Node>` has three nodes `a`, `b`, `c`.
/// Then `c` will have `b` as parent and `b` will have `a` as parent.
/// Node `a` will be returned.
pub fn nest_nodes(nodes: Vec<Vec<ObjectNode>>) -> ObjectNode {
    for node_window in nodes.windows(2) {
        for node in &node_window[0] {
            // Find children node marker in children
            let children_marker_node = node
                .descendants()
                .find(|n| matches!(*n.borrow(), ObjectNodeInner::ChildrenNodeMarker));

            match children_marker_node {
                Some(children_marker_node) => {
                    // Add children to parent of children marker (a marker is always a child of a group)
                    let children_marker_parent = children_marker_node
                        .parent()
                        .expect("Children marker should have a parent");

                    let is_group =
                        matches!(*children_marker_parent.borrow(), ObjectNodeInner::Group(_));

                    if is_group {
                        // Add children to group
                        for node in &node_window[1] {
                            for child in node.children() {
                                children_marker_parent.append(child.clone());
                            }
                        }
                    } else {
                        for node in &node_window[1] {
                            children_marker_parent.append(node.clone());
                        }
                    }
                    // Remove children marker
                    children_marker_node.detach();
                }
                None => {
                    for child in &node_window[1] {
                        node.append(child.clone());
                    }
                }
            }
        }
    }

    if nodes[0].len() == 1 {
        nodes[0].first().expect("Node").clone()
    } else {
        let group = group();
        for node in &nodes[0] {
            group.append(node.clone());
        }
        group
    }
}

/// Dumps the tree structure of a node.
///
/// The depth of a node is marked by the number of white spaces
pub fn dump(writer: &mut dyn std::io::Write, node: ObjectNode) -> std::io::Result<()> {
    use Depth;
    node.descendants()
        .try_for_each(|child| writeln!(writer, "{}{:?}", " ".repeat(child.depth()), child.borrow()))
}

/// Return ObjectNode if we are in a Group
pub fn into_group(node: ObjectNode) -> Option<ObjectNode> {
    node.first_child().and_then(|n| {
        if let ObjectNodeInner::Group(_) = *n.borrow() {
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
            ObjectNodeInner::Group(_) => geo2d::tree::group(),
            ObjectNodeInner::Export(_) => geo2d::tree::group(),
            ObjectNodeInner::Primitive2D(ref renderable) => {
                return Ok(geo2d::tree::geometry(
                    renderable.request_geometry(renderer)?,
                ))
            }
            ObjectNodeInner::Algorithm(ref algorithm) => {
                return algorithm.process_2d(
                    renderer,
                    crate::objects::into_group(node.clone()).unwrap_or(node.clone()),
                )
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
            ObjectNodeInner::Group(_) => geo3d::tree::group(),
            ObjectNodeInner::Export(_) => geo3d::tree::group(),
            ObjectNodeInner::Primitive3D(ref renderable) => {
                return Ok(geo3d::tree::geometry(
                    renderable.request_geometry(renderer)?,
                ))
            }
            ObjectNodeInner::Algorithm(ref algorithm) => {
                return algorithm.process_3d(
                    renderer,
                    crate::objects::into_group(node.clone()).unwrap_or(node.clone()),
                )
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
    let nodes = vec![vec![group()], vec![group()], vec![group()]];
    let node = nest_nodes(nodes.clone());

    nodes[0][0]
        .descendants()
        .for_each(|n| println!("{}{:?}", "  ".repeat(n.depth()), n.borrow()));

    assert_eq!(nodes[2][0].parent().expect("test error"), nodes[1][0]);
    assert_eq!(nodes[1][0].parent().expect("test error"), node);
    assert!(node.parent().is_none());
}

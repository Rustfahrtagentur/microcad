// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render tree

use crate::{export::ExportSettings, geo2d, render::*, Algorithm, Transform};
use strum::IntoStaticStr;

#[cfg(feature = "geo3d")]
use crate::geo3d;

/// Inner of a node
#[derive(IntoStaticStr)]
pub enum NodeInner {
    /// A root node that only contains children
    Root,

    /// A group node that contains children
    Group,

    /// A 2D geometry
    /// This is an rc::Rc to allow for sharing of geometries
    Geometry2D(std::rc::Rc<geo2d::Geometry>),

    /// A generated 2D geometry
    Renderable2D(Box<dyn Renderable2D>),

    /// 3D Geometry
    #[cfg(feature = "geo3d")]
    Geometry3D(std::rc::Rc<geo3d::Geometry>),

    /// Generated 3D geometry
    #[cfg(feature = "geo3d")]
    Renderable3D(Box<dyn Renderable3D>),

    /// An algorithm trait that manipulates the node or its children
    Algorithm(Box<dyn Algorithm>),

    /// An affine transformation of a geometry
    Transform(Transform),

    /// An export node that exports the geometry to a file
    Export(ExportSettings),
}

impl std::fmt::Debug for NodeInner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name: &'static str = self.into();
        write!(f, "{name}")
    }
}

/// Render node
pub type Node = rctree::Node<NodeInner>;

/// Create root node
pub fn root() -> Node {
    Node::new(NodeInner::Root)
}

/// Create new render group
pub fn group() -> Node {
    Node::new(NodeInner::Group)
}

/// Calculate depth trait
pub trait Depth {
    /// Calculate depth
    fn depth(&self) -> usize;
}

impl Depth for Node {
    fn depth(&self) -> usize {
        if let Some(parent) = self.parent() {
            parent.depth() + 1
        } else {
            1
        }
    }
}

/// Nest a Vec of nodes
///
/// Assume, our `Vec<Node` has three nodes `a`, `b`, `c`.
/// Then `c` will have `b` as parent and `b` will have `a` as parent.
/// Node `a` will be returned.
pub fn nest_nodes(nodes: Vec<Node>) -> Node {
    for node_window in nodes.windows(2) {
        node_window[0].append(node_window[1].clone());
    }

    nodes[0].clone()
}

/// Dumps the tree structure of a node.
///
/// The depth of a node is marked by the number of white spaces
pub fn dump(writer: &mut dyn std::io::Write, node: Node) -> std::io::Result<()> {
    node.descendants().try_for_each(|child| {
        writeln!(writer, "{}{:?}", "  ".repeat(child.depth()), child.borrow())
    })
}

#[test]
fn node_nest() {
    let nodes = vec![tree::group(), tree::group(), tree::group()];
    let node = nest_nodes(nodes.clone());

    nodes[0]
        .descendants()
        .for_each(|n| println!("{}{:?}", "  ".repeat(n.depth()), n.borrow()));

    assert_eq!(nodes[2].parent().unwrap(), nodes[1]);
    assert_eq!(nodes[1].parent().unwrap(), node);
    assert!(node.parent().is_none());
}

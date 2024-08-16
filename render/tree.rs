use std::fmt::Debug;

use crate::{geo2d, Renderer};
use microcad_core::*;

pub trait Algorithm {
    fn process(&self, renderer: &dyn Renderer, parent: Node) -> Node;
}

pub struct Transform {
    _mat: Mat4,
}

pub enum NodeInner {
    // A root node that only contains children
    Root,

    /// A group node that contains children
    Group,

    /// A trait that generates a 2D geometry, e.g. a primitive like a circle
    Generator2D(Box<dyn geo2d::Generator>),

    /// A generated geometry
    Geometry2D(Box<geo2d::Geometry>),

    /// An algorithm trait that manipulates the node or its children
    Algorithm(Box<dyn Algorithm>),

    // An affine transformation of a geometry
    Transform(Transform),

    // An export node that exports the geometry to a file
    Export(String),
}

impl Debug for NodeInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeInner::Root => write!(f, "Root"),
            NodeInner::Group => write!(f, "Group"),
            NodeInner::Generator2D(_) => write!(f, "Generator2D"),
            NodeInner::Geometry2D(_) => write!(f, "Geometry2D"),
            NodeInner::Algorithm(_) => write!(f, "Algorithm"),
            NodeInner::Transform(_) => write!(f, "Transform"),
            NodeInner::Export(_) => write!(f, "Export"),
        }
    }
}

pub type Node = rctree::Node<NodeInner>;

pub fn root() -> Node {
    Node::new(NodeInner::Root)
}

pub fn group() -> Node {
    Node::new(NodeInner::Group)
}

pub trait Depth {
    fn depth(&self) -> usize;
}

impl Depth for Node {
    fn depth(&self) -> usize {
        let mut depth = 0;
        let mut node = Some(self.clone());
        while let Some(n) = node {
            depth += 1;
            node = n.parent();
        }
        depth
    }
}

use std::fmt::Debug;

use crate::{geo2d, Error, Renderable2D, Renderer2D};
use microcad_core::*;

pub trait Algorithm {
    fn process_2d(&self, renderer: &mut dyn Renderer2D, parent: Node) -> Result<Node, Error> {
        unimplemented!()
    }
    /*     fn process_3d(
        &self,
        renderer: &dyn Renderer3D,
        parent: Node,
    ) -> Result<Box<dyn Renderable3D>, Error> {
        unimplemented!()
    }*/
}

pub struct Transform {
    _mat: Mat4,
}

pub enum NodeInner {
    // A root node that only contains children
    Root,

    /// A group node that contains children
    Group,

    /// A 2D geometry
    /// This is an rc::Rc to allow for sharing of geometries
    Geometry2D(std::rc::Rc<geo2d::Geometry>),

    /// A generated geometry
    Renderable2D(Box<dyn Renderable2D>),

    /// Changes in render state, given as a key-value pairs
    RenderStateChange(Vec<(String, String)>),

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
            NodeInner::Geometry2D(_) => write!(f, "Geometry2D"),
            NodeInner::Renderable2D(_) => write!(f, "Renderable2D"),
            NodeInner::Algorithm(_) => write!(f, "Algorithm"),
            NodeInner::Transform(_) => write!(f, "Transform"),
            NodeInner::RenderStateChange(_) => write!(f, "RenderStateChange"),
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

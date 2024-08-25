use std::fmt::Debug;

use crate::render::*;

use crate::{export::ExportSettings, geo2d};

#[cfg(feature = "geo3d")]
use crate::geo3d;

pub struct Transform {
    _mat: crate::Mat4,
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

    #[cfg(feature = "geo3d")]
    Geometry3D(std::rc::Rc<geo3d::Geometry>),

    #[cfg(feature = "geo3d")]
    Renderable3D(Box<dyn Renderable3D>),

    /// An algorithm trait that manipulates the node or its children
    Algorithm(Box<dyn crate::Algorithm>),

    // An affine transformation of a geometry
    Transform(Transform),

    // An export node that exports the geometry to a file
    Export(ExportSettings),
}

impl Debug for NodeInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeInner::Root => write!(f, "Root"),
            NodeInner::Group => write!(f, "Group"),
            NodeInner::Geometry2D(_) => write!(f, "Geometry2D"),
            NodeInner::Renderable2D(_) => write!(f, "Renderable2D"),
            #[cfg(feature = "geo3d")]
            NodeInner::Geometry3D(_) => write!(f, "Geometry3D"),
            #[cfg(feature = "geo3d")]
            NodeInner::Renderable3D(_) => write!(f, "Renderable3D"),
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

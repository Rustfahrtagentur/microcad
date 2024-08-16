use crate::{geo2d, Renderer};
use microcad_core::*;

pub trait Algorithm {
    fn process(&self, renderer: &dyn Renderer, parent: Node) -> Node;
}

pub struct Transform {
    _mat: Mat4,
}

pub enum NodeInner {
    // A group node that only contains children
    Group,

    /// A trait that generates a 2D geometry, e.g. a primitive like a circle
    Generator2D(Box<dyn geo2d::Generator>),

    /// A generated geometry
    Geometry2D(Box<geo2d::Geometry>),

    /// An algorithm trait that manipulates the node or its children
    Algorithm(Box<dyn Algorithm>),

    // An affine transformation of a geometry
    Transform(Transform),
}

pub type Node = rctree::Node<NodeInner>;

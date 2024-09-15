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

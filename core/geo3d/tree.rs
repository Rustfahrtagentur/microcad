// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D Geometry Tree
 
use strum::IntoStaticStr;
use super::Geometry;

/// Inner of a node
#[derive(IntoStaticStr)]
pub enum NodeInner {
    /// A group node that contains children
    Group,

    /// 3D Geometry
    Geometry(std::rc::Rc<Geometry>),

    /// An affine transformation of a geometry
    Transform(crate::Mat4),
}

/// Render node
pub type Node = rctree::Node<NodeInner>;

impl crate::Depth for Node {
    fn depth(&self) -> usize {
        if let Some(parent) = self.parent() {
            parent.depth() + 1
        } else {
            0
        }
    }
}

impl std::fmt::Debug for NodeInner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let name: &'static str = self.into();
        write!(f, "{name}")?;

        match &self {
            NodeInner::Transform(transform) => {
                write!(f, "({transform:?})")
            }
            NodeInner::Geometry(geometry) => {
                let geometry_name: &'static str = geometry.as_ref().into();
                write!(f, "({geometry_name})")
            }
            _ => Ok(()),
        }
    }
}

pub fn group() -> Node {
    Node::new(NodeInner::Group)
}

pub fn geometry(geometry: std::rc::Rc<Geometry>) -> Node {
    Node::new(NodeInner::Geometry(geometry))
}

pub fn transform(transform: crate::Mat4) -> Node {
    Node::new(NodeInner::Transform(transform))
}

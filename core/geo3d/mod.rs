// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D Geometry

mod render;
pub mod tree;
mod triangle_mesh;

pub use manifold_rs::Manifold;
pub use triangle_mesh::{Triangle, TriangleMesh, Vertex};

pub use render::*;
pub use tree::{Node, NodeInner};

use crate::BooleanOp;
use strum::IntoStaticStr;

/// 3D Geometry
#[derive(IntoStaticStr)]
pub enum Geometry {
    /// Triangle mesh
    Mesh(TriangleMesh),
    /// Manifold
    Manifold(Manifold),
}

impl From<&BooleanOp> for manifold_rs::BooleanOp {
    fn from(op: &BooleanOp) -> Self {
        match op {
            BooleanOp::Union => manifold_rs::BooleanOp::Union,
            BooleanOp::Intersection => manifold_rs::BooleanOp::Intersection,
            BooleanOp::Difference => manifold_rs::BooleanOp::Difference,
            _ => unimplemented!(),
        }
    }
}

impl Geometry {
    /// Fetch mesh from geometry
    pub fn fetch_mesh(&self) -> TriangleMesh {
        match self {
            Geometry::Mesh(mesh) => mesh.clone(),
            Geometry::Manifold(manifold) => TriangleMesh::from(manifold.to_mesh()),
        }
    }

    /// Execute boolean operation
    pub fn boolean_op(&self, other: &Geometry, op: &BooleanOp) -> Option<Self> {
        let op: manifold_rs::BooleanOp = op.into();
        match (self, other) {
            (Geometry::Mesh(a), Geometry::Mesh(b)) => {
                let result = a.to_manifold().boolean_op(&b.to_manifold(), op);
                Some(Geometry::Manifold(result))
            }
            (Geometry::Manifold(a), Geometry::Manifold(b)) => {
                let result: Manifold = a.boolean_op(b, op);
                Some(Geometry::Manifold(result))
            }
            (Geometry::Mesh(a), Geometry::Manifold(b)) => {
                let result = a.to_manifold().boolean_op(b, op);
                Some(Geometry::Manifold(result))
            }
            (Geometry::Manifold(a), Geometry::Mesh(b)) => {
                let result = a.boolean_op(&b.to_manifold(), op);
                Some(Geometry::Manifold(result))
            }
        }
    }

    /// Execute multiple boolean operations
    pub fn boolean_op_multi(
        geometries: Vec<std::rc::Rc<Self>>,
        op: &BooleanOp,
    ) -> Option<std::rc::Rc<Self>> {
        if geometries.is_empty() {
            return None;
        }

        Some(
            geometries[1..]
                .iter()
                .fold(geometries[0].clone(), |acc, geo| {
                    if let Some(r) = acc.boolean_op(geo.as_ref(), op) {
                        std::rc::Rc::new(r)
                    } else {
                        acc
                    }
                }),
        )
    }

    /// Transform mesh geometry
    pub fn transform(&self, transform: crate::Mat4) -> Self {
        match self {
            Geometry::Mesh(mesh) => Geometry::Mesh(mesh.transform(transform)),
            Geometry::Manifold(manifold) => {
                // TODO: Implement transform for manifold instead of converting to mesh
                Geometry::Mesh(TriangleMesh::from(manifold.to_mesh()).transform(transform))
            }
        }
    }
}

impl From<Manifold> for Geometry {
    fn from(manifold: Manifold) -> Self {
        Geometry::Manifold(manifold)
    }
}

impl From<TriangleMesh> for Geometry {
    fn from(mesh: TriangleMesh) -> Self {
        Geometry::Mesh(mesh)
    }
}

/// Create a new group node
pub fn group() -> Node {
    Node::new(NodeInner::Group)
}

/// Create a new geometry node
pub fn geometry(geometry: std::rc::Rc<Geometry>) -> Node {
    Node::new(NodeInner::Geometry(geometry))
}

/// Create a new transform node
pub fn transform(transform: crate::Mat4) -> Node {
    Node::new(NodeInner::Transform(transform))
}

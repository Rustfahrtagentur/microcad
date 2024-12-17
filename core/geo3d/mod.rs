// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D Geometry

mod mesh_renderer;
mod render;
pub mod tree;
mod triangle_mesh;

pub use manifold_rs::Manifold;
pub use triangle_mesh::{Triangle, TriangleMesh, Vertex};

pub use mesh_renderer::MeshRenderer;

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
    /// Get the volume of the geometry
    pub fn volume(&self) -> f64 {
        match self {
            Geometry::Mesh(mesh) => mesh.volume(),
            Geometry::Manifold(manifold) => TriangleMesh::from(manifold.to_mesh()).volume(),
        }
    }

    /// Execute boolean operation
    pub fn boolean_op(&self, other: &Geometry, op: &BooleanOp) -> Option<Self> {
        let op: manifold_rs::BooleanOp = op.into();
        Some(Geometry::Manifold(match (self, other) {
            (Geometry::Mesh(a), Geometry::Mesh(b)) => {
                a.to_manifold().boolean_op(&b.to_manifold(), op)
            }
            (Geometry::Manifold(a), Geometry::Manifold(b)) => a.boolean_op(b, op),
            (Geometry::Mesh(a), Geometry::Manifold(b)) => a.to_manifold().boolean_op(b, op),
            (Geometry::Manifold(a), Geometry::Mesh(b)) => a.boolean_op(&b.to_manifold(), op),
        }))
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
    pub fn transform(&self, transform: &crate::Mat4) -> Self {
        match self {
            Geometry::Mesh(mesh) => Geometry::Mesh(mesh.transform(transform)),

            Geometry::Manifold(manifold) => {
                // TODO: Implement transform for manifold instead of converting to mesh
                let mesh = TriangleMesh::from(manifold.to_mesh()).transform(transform);
                Geometry::Manifold(mesh.to_manifold())
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

#[test]
fn test_boolean_op_multi() {
    let a = std::rc::Rc::new(Geometry::Manifold(Manifold::sphere(2.0, 32)));
    let b = std::rc::Rc::new(Geometry::Manifold(Manifold::sphere(1.0, 32)));

    let result = Geometry::boolean_op_multi(vec![a, b], &BooleanOp::Difference);
    assert!(result.is_some());

    let result = result.expect("test error");

    if let Geometry::Manifold(manifold) = &*result {
        assert!(manifold.to_mesh().vertices().len() > 1);
    } else {
        panic!("Expected manifold");
    }

    let transform = crate::Mat4::from_translation(crate::Vec3::new(5.0, 10.0, 0.0));
    let result = result.transform(&transform);

    if let Geometry::Manifold(manifold) = result {
        assert!(manifold.to_mesh().vertices().len() > 1);
    } else {
        panic!("Expected manifold");
    }
}

#[test]
fn test_mesh_volume() {
    let manifold = Manifold::sphere(1.0, 512);
    let mesh = TriangleMesh::from(manifold.to_mesh());

    let volume = mesh.volume();
    assert!((volume - 4.0 / 3.0 * std::f64::consts::PI).abs() < 1e-3);
}

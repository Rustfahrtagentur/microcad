// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

pub use super::*;
use std::rc::Rc;
use strum::IntoStaticStr;

/// 3D Geometry
#[derive(IntoStaticStr)]
pub enum Geometry {
    /// Triangle mesh
    Mesh(TriangleMesh),
    /// Manifold
    Manifold(Manifold),
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
    pub fn boolean_op_multi(geometries: Vec<Rc<Self>>, op: &BooleanOp) -> Option<Rc<Self>> {
        if geometries.is_empty() {
            return None;
        }

        Some(
            geometries[1..]
                .iter()
                .fold(geometries[0].clone(), |acc, geo| {
                    if let Some(r) = acc.boolean_op(geo.as_ref(), op) {
                        Rc::new(r)
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

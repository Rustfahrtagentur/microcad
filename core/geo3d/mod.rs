// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D Geometry

mod bounds;
mod collection;
mod extrude;
mod geometry;
mod mesh;
mod primitives;

pub use bounds::*;
pub use collection::*;
pub use extrude::*;
pub use geometry::*;
pub use manifold_rs::Manifold;
pub use mesh::{Triangle, TriangleMesh, Vertex};
pub use primitives::*;

use crate::{BooleanOp, RenderResolution};

impl From<&BooleanOp> for manifold_rs::BooleanOp {
    fn from(op: &BooleanOp) -> Self {
        match op {
            BooleanOp::Union => manifold_rs::BooleanOp::Union,
            BooleanOp::Intersect => manifold_rs::BooleanOp::Intersection,
            BooleanOp::Subtract => manifold_rs::BooleanOp::Difference,
            _ => unimplemented!(),
        }
    }
}

/// Trait to render a 3D geometry into a mesh.
pub trait RenderToMesh: Sized {
    /// Render to manifold.
    ///
    /// Implement this method preferably.
    fn render_to_manifold(&self, resolution: &RenderResolution) -> std::rc::Rc<Manifold>;

    /// Render to mesh.
    ///
    /// Implement only if [`RenderToMesh::render_to_manifold`] is not possible.
    fn render_to_mesh(&self, resolution: &RenderResolution) -> TriangleMesh {
        self.render_to_manifold(resolution).to_mesh().into()
    }
}

#[test]
fn test_mesh_volume() {
    let manifold = Manifold::sphere(1.0, 512);
    let mesh = TriangleMesh::from(manifold.to_mesh());

    let volume = mesh.volume();
    assert!((volume - 4.0 / 3.0 * std::f64::consts::PI).abs() < 1e-3);
}

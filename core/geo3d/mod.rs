// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D Geometry

mod bounds;
mod collection;
mod geometry;
mod primitives;
mod triangle_mesh;

pub use bounds::*;
pub use collection::*;
pub use geometry::*;
pub use manifold_rs::Manifold;
pub use primitives::*;
pub use triangle_mesh::{Triangle, TriangleMesh, Vertex};

use crate::{BooleanOp, RenderResolution};

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
fn test_boolean_op_multi() {
    use std::rc::Rc;
    let a = Rc::new(Geometry3D::Manifold(std::rc::Rc::new(Manifold::sphere(
        2.0, 32,
    ))));
    let b = Rc::new(Geometry3D::Manifold(std::rc::Rc::new(Manifold::sphere(
        1.0, 32,
    ))));

    let result = Geometry3D::boolean_op_multi(
        vec![a, b],
        &RenderResolution::default(),
        &BooleanOp::Difference,
    );
    assert!(result.is_some());

    let result = result.expect("test error");

    if let Geometry3D::Manifold(manifold) = &*result {
        assert!(manifold.to_mesh().vertices().len() > 1);
    } else {
        panic!("Expected manifold");
    }

    let transform = crate::Mat4::from_translation(crate::Vec3::new(5.0, 10.0, 0.0));
    let result = result.transform(&transform);

    if let Geometry3D::Manifold(manifold) = result {
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

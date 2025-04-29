// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D Geometry

mod geometry;
mod mesh_renderer;
mod render;
pub mod tree;
mod triangle_mesh;

pub use manifold_rs::Manifold;
pub use triangle_mesh::{Triangle, TriangleMesh, Vertex};

pub use mesh_renderer::MeshRenderer;

pub use geometry::*;
pub use render::*;
pub use tree::{Node, NodeInner};

use crate::BooleanOp;

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

/// Create a new group node
pub fn group() -> Node {
    Node::new(NodeInner::Group)
}

/// Create a new transform node
pub fn transform(transform: crate::Mat4) -> Node {
    Node::new(NodeInner::Transform(transform))
}

#[test]
fn test_boolean_op_multi() {
    use std::rc::Rc;
    let a = Rc::new(Geometry::Manifold(Manifold::sphere(2.0, 32)));
    let b = Rc::new(Geometry::Manifold(Manifold::sphere(1.0, 32)));

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

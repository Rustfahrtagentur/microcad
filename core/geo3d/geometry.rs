// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::*;

use std::rc::Rc;
use strum::IntoStaticStr;

use crate::geo3d::*;

/// 3D Geometry
#[derive(IntoStaticStr, Clone)]
pub enum Geometry3D {
    /// Triangle mesh.
    Mesh(TriangleMesh),
    /// Manifold.
    Manifold(Rc<Manifold>),
    /// Cube.
    Cube(Cube),
    /// Sphere.
    Sphere(Sphere),
    /// Cylinder.
    Cylinder(Cylinder),
}

impl std::fmt::Debug for Geometry3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &'static str = self.into();
        write!(f, "{name}")
    }
}

impl Geometry3D {
    /// Execute boolean operation
    pub fn boolean_op(
        &self,
        resolution: &RenderResolution,
        other: &Geometry3D,
        op: &BooleanOp,
    ) -> Option<Self> {
        let op: manifold_rs::BooleanOp = op.into();
        let a = self.clone().render_to_manifold(resolution);
        let b = other.clone().render_to_manifold(resolution);

        Some(Geometry3D::Manifold(Rc::new(a.boolean_op(&b, op))))
    }

    /// Execute multiple boolean operations
    pub fn boolean_op_multi(
        geometries: Vec<Rc<Self>>,
        resolution: &RenderResolution,
        op: &BooleanOp,
    ) -> Option<Rc<Self>> {
        if geometries.is_empty() {
            return None;
        }

        Some(
            geometries[1..]
                .iter()
                .fold(geometries[0].clone(), |acc, geo| {
                    if let Some(r) = acc.boolean_op(resolution, geo.as_ref(), op) {
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
            Geometry3D::Mesh(mesh) => Geometry3D::Mesh(mesh.transform(transform)),

            Geometry3D::Manifold(manifold) => {
                // TODO: Implement transform for manifold instead of converting to mesh
                let mesh = TriangleMesh::from(manifold.to_mesh()).transform(transform);
                Geometry3D::Manifold(Rc::new(mesh.to_manifold()))
            }
            _ => todo!(),
        }
    }
}

impl FetchBounds3D for Rc<Manifold> {
    fn fetch_bounds_3d(&self) -> Bounds3D {
        todo!()
    }
}

impl FetchBounds3D for Geometry3D {
    fn fetch_bounds_3d(&self) -> Bounds3D {
        match self {
            Geometry3D::Mesh(triangle_mesh) => triangle_mesh.fetch_bounds_3d(),
            Geometry3D::Manifold(manifold) => manifold.fetch_bounds_3d(),
            Geometry3D::Cube(cube) => cube.fetch_bounds_3d(),
            Geometry3D::Sphere(sphere) => sphere.fetch_bounds_3d(),
            Geometry3D::Cylinder(cylinder) => cylinder.fetch_bounds_3d(),
        }
    }
}

impl RenderToMesh for Geometry3D {
    fn render_to_manifold(self, resolution: &RenderResolution) -> Rc<Manifold> {
        match self {
            Geometry3D::Mesh(triangle_mesh) => Rc::new(triangle_mesh.to_manifold()),
            Geometry3D::Manifold(manifold) => manifold,
            Geometry3D::Cube(cube) => cube.render_to_manifold(resolution),
            Geometry3D::Sphere(sphere) => sphere.render_to_manifold(resolution),
            Geometry3D::Cylinder(cylinder) => cylinder.render_to_manifold(resolution),
        }
    }
}

impl From<TriangleMesh> for Geometry3D {
    fn from(mesh: TriangleMesh) -> Self {
        Geometry3D::Mesh(mesh)
    }
}

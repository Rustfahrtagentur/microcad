// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D Geometry collection

use std::rc::Rc;

use derive_more::{Deref, DerefMut};

use crate::{
    geo3d::{FetchBounds3D, bounds::Bounds3D},
    *,
};

/// 3D geometry collection.
#[derive(Debug, Clone, Default, Deref, DerefMut, serde::Serialize, serde::Deserialize)]
pub struct Geometries3D(Vec<Rc<Geometry3D>>);

impl Geometries3D {
    /// New geometry collection.
    pub fn new(geometries: Vec<Rc<Geometry3D>>) -> Self {
        Self(geometries)
    }

    /// Append another geometry collection.
    pub fn append(&mut self, mut geometries: Geometries3D) {
        self.0.append(&mut geometries.0)
    }

    /// Apply boolean operation to geometry collection.
    pub fn boolean_op(&self, resolution: &RenderResolution, op: &BooleanOp) -> Geometries3D {
        if self.0.is_empty() {
            return Geometries3D::default();
        }

        self.0[1..]
            .iter()
            .fold(self.0[0].clone(), |acc, geo| {
                if let Some(r) = acc.boolean_op(resolution, geo.as_ref(), op) {
                    Rc::new(r)
                } else {
                    acc
                }
            })
            .into()
    }
}

impl FetchBounds3D for Geometries3D {
    fn fetch_bounds_3d(&self) -> Bounds3D {
        self.0.iter().fold(Bounds3D::default(), |bounds, geometry| {
            bounds.extend(geometry.fetch_bounds_3d())
        })
    }
}

impl Transformed3D for Geometries3D {
    fn transformed_3d(&self, render_resolution: &RenderResolution, mat: &Mat4) -> Self {
        Self(
            self.iter()
                .map(|geometry| std::rc::Rc::new(geometry.transformed_3d(render_resolution, mat)))
                .collect::<Vec<_>>(),
        )
    }
}

impl RenderToMesh for Geometries3D {
    fn render_to_manifold(self, resolution: &RenderResolution) -> std::rc::Rc<Manifold> {
        std::rc::Rc::new(self.render_to_mesh(resolution).to_manifold())
    }

    fn render_to_mesh(self, resolution: &RenderResolution) -> TriangleMesh {
        self.iter().fold(TriangleMesh::default(), |mut mesh, geo| {
            mesh.append(&geo.as_ref().clone().render_to_mesh(resolution));
            mesh
        })
    }
}

impl From<std::rc::Rc<Geometry3D>> for Geometries3D {
    fn from(geometry: std::rc::Rc<Geometry3D>) -> Self {
        Self::new(vec![geometry])
    }
}

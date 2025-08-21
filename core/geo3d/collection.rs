// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D Geometry collection

use derive_more::{Deref, DerefMut};

use crate::{
    geo3d::{FetchBounds3D, bounds::Bounds3D},
    *,
};

/// 3D geometry collection.
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct Geometries3D(Vec<Geometry3D>);

impl Geometries3D {
    /// New geometry collection.
    pub fn new(geometries: Vec<Geometry3D>) -> Self {
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
            .fold(self.0[0].clone(), |acc, other| {
                if let Some(r) = acc.boolean_op(resolution, other, op) {
                    r
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
                .map(|geometry| geometry.transformed_3d(render_resolution, mat))
                .collect::<Vec<_>>(),
        )
    }
}

impl From<Geometry3D> for Geometries3D {
    fn from(geometry: Geometry3D) -> Self {
        Self::new(vec![geometry])
    }
}

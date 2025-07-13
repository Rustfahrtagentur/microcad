// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D Geometry collection

use std::rc::Rc;

use crate::{
    geo3d::{FetchBounds3D, bounds::Bounds3D},
    *,
};

/// 2D geometry collection with bounding box.
#[derive(Debug, Clone, Default)]
pub struct Geometries3D {
    /// Geometries.
    geometries: Vec<Rc<Geometry3D>>,
    /// Bounding rect.
    bounds: Bounds3D,
}

impl Geometries3D {
    /// New geometry collection.
    pub fn new(geometries: Vec<Rc<Geometry3D>>) -> Self {
        let bounds: Bounds3D = geometries.iter().fold(Bounds3D::default(), |acc, e| {
            acc.extend(e.fetch_bounds_3d())
        });

        Self { geometries, bounds }
    }

    /// Push a new geometry to the collection and update bounding box.
    pub fn push(&mut self, geometry: Rc<Geometry3D>) {
        self.bounds = self.bounds.clone().extend(geometry.fetch_bounds_3d());
        self.geometries.push(geometry)
    }
}

impl FetchBounds3D for Geometries3D {
    fn fetch_bounds_3d(&self) -> Bounds3D {
        self.bounds.clone()
    }
}

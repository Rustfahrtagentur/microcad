// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry collection

use std::rc::Rc;

use crate::{
    geo2d::{FetchBounds2D, bounds::Bounds2D},
    *,
};

/// 2D geometry collection with bounding box.
#[derive(Debug, Clone, Default)]
pub struct Geometries2D {
    /// Geometries.
    geometries: Vec<Rc<Geometry2D>>,
    /// Bounding rect.
    bounds: Bounds2D,
}

impl Geometries2D {
    /// New geometry collection.
    pub fn new(geometries: Vec<Rc<Geometry2D>>) -> Self {
        let bounds: Bounds2D = geometries.iter().fold(Bounds2D::default(), |acc, e| {
            acc.extend(e.fetch_bounds_2d())
        });

        Self { geometries, bounds }
    }

    /// Push a new geometry to the collection and update bounding box.
    pub fn push(&mut self, geometry: Rc<Geometry2D>) {
        self.bounds = self.bounds.clone().extend(geometry.fetch_bounds_2d());
        self.geometries.push(geometry)
    }
}

impl FetchBounds2D for Geometries2D {
    fn fetch_bounds_2d(&self) -> Bounds2D {
        self.bounds.clone()
    }
}

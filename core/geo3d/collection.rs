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
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct Geometries3D(Vec<Rc<Geometry3D>>);

impl Geometries3D {
    /// New geometry collection.
    pub fn new(geometries: Vec<Geometry3D>) -> Self {
        Self(geometries.into_iter().map(Rc::new).collect())
    }

    /// Append another geometry collection.
    pub fn append(&mut self, mut geometries: Geometries3D) {
        self.0.append(&mut geometries.0)
    }

    /// Apply boolean operation on collection and render to manifold.
    pub fn boolean_op(&self, op: &BooleanOp) -> Rc<Manifold> {
        let manifold_list: Vec<_> = self
            .0
            .iter()
            // Render each geometry into a multipolygon and filter out empty ones
            .filter_map(|geo| {
                let manifold: Rc<Manifold> = geo.as_ref().clone().into();
                if manifold.is_empty() {
                    None
                } else {
                    Some(manifold)
                }
            })
            .collect();

        if manifold_list.is_empty() {
            return Rc::new(Manifold::empty());
        }

        manifold_list[1..]
            .iter()
            .fold(manifold_list[0].clone(), |acc, other| {
                Rc::new(acc.boolean_op(other, op.into()))
            })
    }
}

impl FromIterator<Rc<Geometry3D>> for Geometries3D {
    fn from_iter<T: IntoIterator<Item = Rc<Geometry3D>>>(iter: T) -> Self {
        Geometries3D(iter.into_iter().collect())
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
    fn transformed_3d(&self, mat: &Mat4) -> Self {
        Self(
            self.iter()
                .map(|geometry| Rc::new(geometry.transformed_3d(mat)))
                .collect::<Vec<_>>(),
        )
    }
}

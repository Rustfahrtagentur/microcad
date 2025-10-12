// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry collection

use derive_more::{Deref, DerefMut};
use geo::{CoordsIter, HasDimensions, MultiPolygon};
use std::rc::Rc;

use crate::{
    geo2d::{CalcBounds2D, bounds::Bounds2D},
    *,
};

/// 2D geometry collection.
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct Geometries2D(Vec<Rc<Geometry2D>>);

impl Geometries2D {
    /// New geometry collection.
    pub fn new(geometries: Vec<Geometry2D>) -> Self {
        Self(geometries.into_iter().map(Rc::new).collect())
    }

    /// Append another geometry collection.
    pub fn append(&mut self, mut geometries: Geometries2D) {
        self.0.append(&mut geometries.0)
    }

    /// Apply boolean operation to render into MultiPolygon.
    pub fn boolean_op(&self, op: &BooleanOp) -> geo2d::MultiPolygon {
        let multi_polygon_list: Vec<_> = self
            .0
            .iter()
            // Render each geometry into a multipolygon and filter out empty ones
            .filter_map(|geo| {
                let multi_polygon = geo.to_multi_polygon();
                if multi_polygon.is_empty() {
                    None
                } else {
                    Some(multi_polygon)
                }
            })
            .collect();

        if multi_polygon_list.is_empty() {
            return geo2d::MultiPolygon::empty();
        }

        multi_polygon_list[1..]
            .iter()
            .fold(multi_polygon_list[0].clone(), |acc, geo| {
                use geo::BooleanOps;
                acc.boolean_op(geo, op.into())
            })
    }

    /// Generate multipolygon.
    pub fn to_multi_polygon(&self) -> MultiPolygon {
        let mut polygons = Vec::new();
        self.iter().for_each(|geo| {
            polygons.append(&mut (**geo).clone().to_multi_polygon().0);
        });

        MultiPolygon::new(polygons)
    }

    /// Apply contex hull operation to geometries.
    pub fn hull(&self) -> geo2d::Polygon {
        let mut coords = self.iter().fold(Vec::new(), |mut coords, geo| {
            match geo.as_ref() {
                Geometry2D::LineString(line_string) => {
                    coords.append(&mut line_string.coords_iter().collect())
                }
                Geometry2D::MultiLineString(multi_line_string) => {
                    coords.append(&mut multi_line_string.coords_iter().collect())
                }
                Geometry2D::Polygon(polygon) => {
                    coords.append(&mut polygon.exterior_coords_iter().collect())
                }
                Geometry2D::MultiPolygon(multi_polygon) => {
                    coords.append(&mut multi_polygon.exterior_coords_iter().collect())
                }
                Geometry2D::Rect(rect) => {
                    let mut rect_corners: Vec<_> = rect.coords_iter().collect();
                    coords.append(&mut rect_corners)
                }
                Geometry2D::Line(line) => {
                    coords.push(line.0.into());
                    coords.push(line.1.into());
                }
                Geometry2D::Collection(collection) => {
                    coords.append(&mut collection.hull().exterior_coords_iter().collect())
                }
            }
            coords
        });

        geo2d::Polygon::new(
            geo::algorithm::convex_hull::qhull::quick_hull(&mut coords),
            vec![],
        )
    }
}

impl FromIterator<Rc<Geometry2D>> for Geometries2D {
    fn from_iter<T: IntoIterator<Item = Rc<Geometry2D>>>(iter: T) -> Self {
        Geometries2D(iter.into_iter().collect())
    }
}

impl CalcBounds2D for Geometries2D {
    fn calc_bounds_2d(&self) -> Bounds2D {
        self.0.iter().fold(Bounds2D::default(), |bounds, geometry| {
            bounds.extend(geometry.calc_bounds_2d())
        })
    }
}

impl Transformed2D for Geometries2D {
    fn transformed_2d(&self, mat: &Mat3) -> Self {
        Self(
            self.iter()
                .map(|geometry| Rc::new(geometry.transformed_2d(mat)))
                .collect::<Vec<_>>(),
        )
    }
}

impl From<Geometries2D> for MultiPolygon {
    fn from(geometries: Geometries2D) -> Self {
        Self(
            geometries
                .iter()
                .flat_map(|geo| {
                    let multi_polygon: MultiPolygon = geo.as_ref().clone().into();
                    multi_polygon.0
                })
                .collect(),
        )
    }
}

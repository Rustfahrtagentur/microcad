// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry collection

use std::rc::Rc;

use derive_more::{Deref, DerefMut};
use geo::{CoordsIter, LineString, Polygon};

use crate::{
    geo2d::{FetchBounds2D, bounds::Bounds2D},
    *,
};

/// 2D geometry collection.
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct Geometries2D(Vec<Rc<Geometry2D>>);

impl Geometries2D {
    /// New geometry collection.
    pub fn new(geometries: Vec<Rc<Geometry2D>>) -> Self {
        Self(geometries)
    }

    /// Append another geometry collection.
    pub fn append(&mut self, mut geometries: Geometries2D) {
        self.0.append(&mut geometries.0)
    }

    /// Apply boolean operation to multiple geometries.
    pub fn boolean_op(&self, resolution: &RenderResolution, op: &BooleanOp) -> Self {
        if self.0.is_empty() {
            return Geometries2D::default();
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

    /// Apply contex hull operation to geometries.
    pub fn hull(&self, resolution: &RenderResolution) -> Self {
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
                Geometry2D::Circle(circle) => coords.append(
                    &mut circle
                        .clone()
                        .render_to_polygon(resolution)
                        .unwrap_or(Polygon::new(LineString(vec![]), vec![]))
                        .exterior_coords_iter()
                        .collect(),
                ),
                Geometry2D::Line(line) => {
                    coords.push(line.0.into());
                    coords.push(line.1.into());
                }
            }
            coords
        });

        Rc::new(Geometry2D::Polygon(geo2d::Polygon::new(
            geo::algorithm::convex_hull::qhull::quick_hull(&mut coords),
            vec![],
        )))
        .into()
    }
}

impl FetchBounds2D for Geometries2D {
    fn fetch_bounds_2d(&self) -> Bounds2D {
        self.0.iter().fold(Bounds2D::default(), |bounds, geometry| {
            bounds.extend(geometry.fetch_bounds_2d())
        })
    }
}

impl Transformed2D for Geometries2D {
    fn transformed_2d(&self, render_resolution: &RenderResolution, mat: &Mat3) -> Self {
        Self(
            self.iter()
                .map(|geometry| std::rc::Rc::new(geometry.transformed_2d(render_resolution, mat)))
                .collect::<Vec<_>>(),
        )
    }
}

impl From<std::rc::Rc<Geometry2D>> for Geometries2D {
    fn from(geometry: std::rc::Rc<Geometry2D>) -> Self {
        Self::new(vec![geometry])
    }
}

impl RenderToMultiPolygon for Geometries2D {
    fn render_to_existing_multi_polygon(
        self,
        resolution: &RenderResolution,
        polygons: &mut geo2d::MultiPolygon,
    ) {
        self.iter().for_each(|geometry| {
            geometry
                .as_ref()
                .clone()
                .render_to_existing_multi_polygon(resolution, polygons);
        });
    }
}

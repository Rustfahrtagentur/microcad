// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry

pub mod tree;

mod geometry;
mod render;

use crate::Scalar;

pub use geometry::*;
pub use render::*;

/// Line string
pub type LineString = geo::LineString<Scalar>;
/// Multiple line string
pub type MultiLineString = geo::MultiLineString<Scalar>;
/// Polygon
pub type Polygon = geo::Polygon<Scalar>;
/// Multiple polygons
pub type MultiPolygon = geo::MultiPolygon<Scalar>;
/// Rectangle
pub type Rect = geo::Rect<Scalar>;
/// Point
pub type Point = geo::Point<Scalar>;

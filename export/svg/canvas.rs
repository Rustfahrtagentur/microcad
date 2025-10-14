// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Canvas to draw geometry.

use geo::MultiPolygon;
use microcad_core::{
    Bounds2D, Circle, Geometries2D, Geometry2D, Line, LineString, MultiLineString, Point, Polygon,
    Rect, Scalar, Size2, Vec2, geo2d,
};

use crate::svg::CenteredText;

/// A canvas coordinate system.
#[derive(Clone, Debug)]
pub struct Canvas {
    /// The canvas rect.
    pub rect: geo2d::Rect,
    /// The content rect.
    pub content_rect: geo2d::Rect,
    /// Size2.
    pub size: Size2,
}

impl Canvas {
    /// Create a new canvas with a size and center the content.
    pub fn new_centered_content(
        size: Size2,
        content_rect: geo2d::Rect,
        scale: Option<Scalar>,
    ) -> Self {
        // Compute scale to fit content inside canvas (preserving aspect ratio)
        let scale = match scale {
            Some(scale) => scale,
            None => {
                let scale_x = size.width / content_rect.width();
                let scale_y = size.height / content_rect.height();
                scale_x.min(scale_y)
            }
        };

        // New content size after scaling
        let width = content_rect.width() * scale;
        let height = content_rect.height() * scale;

        // Center the content within the canvas
        let min = Point::new((size.width - width) / 2.0, (size.height - height) / 2.0);

        // Build the new canvas rect centered with content
        let rect = geo2d::Rect::new(min, min + geo2d::Point::new(width, height));

        Canvas {
            rect,
            content_rect,
            size,
        }
    }

    /// Return the ratio between canvas rect and content rect size.
    pub fn scale(&self) -> Scalar {
        (self.rect.width() / self.content_rect.width())
            .min(self.rect.height() / self.content_rect.height())
    }
}

/// Map something into a canvas coordinates.
pub trait MapToCanvas: Sized {
    /// Return mapped version.
    fn map_to_canvas(&self, canvas: &Canvas) -> Self;
}

/// Scale scalar value.
impl MapToCanvas for Scalar {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        self * canvas.scale()
    }
}

impl MapToCanvas for (Scalar, Scalar) {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        let scale = canvas.scale();
        let new_width = canvas.rect.width() / scale;
        let new_height = canvas.rect.height() / scale;

        let x = self.0 - canvas.content_rect.min().x;
        let y = canvas.content_rect.max().y - self.1; // Flip Y
        let x = x / new_width * canvas.rect.width() + canvas.rect.min().x;
        let y = y / new_height * canvas.rect.height() + canvas.rect.min().y;
        (x, y)
    }
}

impl MapToCanvas for Point {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        Point::from(self.x_y().map_to_canvas(canvas))
    }
}

impl MapToCanvas for Vec2 {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        Vec2::from((self.x, self.y).map_to_canvas(canvas))
    }
}

impl MapToCanvas for Line {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        Self(self.0.map_to_canvas(canvas), self.1.map_to_canvas(canvas))
    }
}

impl MapToCanvas for Rect {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        Self::new(
            Point::from(self.min()).map_to_canvas(canvas),
            Point::from(self.max()).map_to_canvas(canvas),
        )
    }
}

impl MapToCanvas for Bounds2D {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        match self.rect() {
            Some(rect) => Self::new(
                rect.min().x_y().map_to_canvas(canvas).into(),
                rect.max().x_y().map_to_canvas(canvas).into(),
            ),
            None => Self::default(),
        }
    }
}

impl MapToCanvas for Circle {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        Self {
            radius: self.radius.map_to_canvas(canvas),
            offset: self.offset.map_to_canvas(canvas),
        }
    }
}

impl MapToCanvas for LineString {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        Self(
            self.0
                .iter()
                .map(|p| p.x_y().map_to_canvas(canvas).into())
                .collect(),
        )
    }
}

impl MapToCanvas for MultiLineString {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        self.iter()
            .map(|line_string| line_string.map_to_canvas(canvas))
            .collect()
    }
}

impl MapToCanvas for Polygon {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        Self::new(
            self.exterior().map_to_canvas(canvas),
            self.interiors()
                .iter()
                .map(|line_string| line_string.map_to_canvas(canvas))
                .collect(),
        )
    }
}

impl MapToCanvas for MultiPolygon {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        self.0
            .iter()
            .map(|polygon| polygon.map_to_canvas(canvas))
            .collect()
    }
}

impl MapToCanvas for Geometries2D {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        Geometries2D::new(self.iter().map(|geo| geo.map_to_canvas(canvas)).collect())
    }
}

impl MapToCanvas for Geometry2D {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        match self {
            Geometry2D::LineString(line_string) => {
                Geometry2D::LineString(line_string.map_to_canvas(canvas))
            }
            Geometry2D::MultiLineString(multi_line_string) => {
                Geometry2D::MultiLineString(multi_line_string.map_to_canvas(canvas))
            }
            Geometry2D::Polygon(polygon) => Geometry2D::Polygon(polygon.map_to_canvas(canvas)),
            Geometry2D::MultiPolygon(multi_polygon) => {
                Geometry2D::MultiPolygon(multi_polygon.map_to_canvas(canvas))
            }
            Geometry2D::Rect(rect) => Geometry2D::Rect(rect.map_to_canvas(canvas)),
            Geometry2D::Line(edge) => Geometry2D::Line(edge.map_to_canvas(canvas)),
            Geometry2D::Collection(collection) => {
                Geometry2D::Collection(collection.map_to_canvas(canvas))
            }
        }
    }
}

impl MapToCanvas for CenteredText {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        CenteredText {
            text: self.text.clone(),
            rect: self.rect.map_to_canvas(canvas),
            font_size: self.font_size,
        }
    }
}

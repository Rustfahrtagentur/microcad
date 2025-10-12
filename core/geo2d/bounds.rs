// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry bounds.

use derive_more::Deref;
use geo::coord;

use crate::*;

/// Bounds2D type alias.
pub type Bounds2D = Bounds<Vec2>;

impl Bounds2D {
    /// Check if bounds are valid.
    pub fn is_valid(&self) -> bool {
        self.min.x <= self.max.x && self.min.y <= self.max.y
    }

    /// Calculate width of these bounds.
    pub fn width(&self) -> Scalar {
        (self.max.x - self.min.x).max(0.0)
    }

    /// Calculate height of these bounds.
    pub fn height(&self) -> Scalar {
        (self.max.y - self.min.y).max(0.0)
    }

    /// Maximum of width and height.
    pub fn max_extent(&self) -> Scalar {
        self.width().max(self.height())
    }

    /// Return rect.
    pub fn rect(&self) -> Option<Rect> {
        if self.is_valid() {
            Some(Rect::new(
                coord! {x: self.min.x, y: self.min.y },
                coord! {x: self.max.x, y: self.max.y },
            ))
        } else {
            None
        }
    }

    /// Enlarge bounds by a factor and return new bounds
    pub fn enlarge(&self, factor: Scalar) -> Self {
        match self.rect() {
            Some(rect) => {
                let c = rect.center();
                let s: geo::Coord = (rect.width(), rect.height()).into();
                let s = s * 0.5 * (1.0 + factor);
                Rect::new(c - s, c + s).into()
            }
            None => Bounds2D::default(),
        }
    }

    /// Calculate extended bounds.
    pub fn extend(mut self, other: Bounds2D) -> Self {
        self.extend_by_point(other.min);
        self.extend_by_point(other.max);
        self
    }

    /// Extend these bounds by point.
    pub fn extend_by_point(&mut self, p: Vec2) {
        self.min.x = p.x.min(self.min.x);
        self.min.y = p.y.min(self.min.y);
        self.max.x = p.x.max(self.max.x);
        self.max.y = p.y.max(self.max.y);
    }
}

impl Default for Bounds2D {
    fn default() -> Self {
        // Bounds are invalid by default.
        let min = Scalar::MAX;
        let max = Scalar::MIN;
        Self::new((min, min).into(), (max, max).into())
    }
}

impl From<Rect> for Bounds2D {
    fn from(rect: Rect) -> Self {
        Self::new(rect.min().x_y().into(), rect.max().x_y().into())
    }
}

impl From<Option<Rect>> for Bounds2D {
    fn from(rect: Option<Rect>) -> Self {
        match rect {
            Some(rect) => rect.into(),
            None => Bounds2D::default(),
        }
    }
}

impl From<Size2> for Bounds2D {
    fn from(value: Size2) -> Self {
        Self::new(Vec2::new(0.0, 0.0), Vec2::new(value.width, value.height))
    }
}

impl std::fmt::Display for Bounds2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.rect() {
            Some(rect) => write!(
                f,
                "[{min:?}, {max:?}]",
                min = rect.min().x_y(),
                max = rect.max().x_y()
            ),
            None => write!(f, "[no bounds]"),
        }
    }
}

/// Trait to calculate a bounding box of 2D geometry.
pub trait CalcBounds2D {
    /// Fetch bounds.
    fn calc_bounds_2d(&self) -> Bounds2D;
}

/// Holds bounds for a 3D object.
#[derive(Clone, Default, Debug, Deref)]
pub struct WithBounds2D<T: CalcBounds2D + Transformed2D> {
    /// Bounds.
    pub bounds: Bounds2D,
    /// The inner object.
    #[deref]
    pub inner: T,
}

impl<T: CalcBounds2D + Transformed2D> WithBounds2D<T> {
    /// Create a new object with bounds.
    pub fn new(inner: T) -> Self {
        Self {
            bounds: inner.calc_bounds_2d(),
            inner,
        }
    }

    /// Update the bounds.
    pub fn update_bounds(&mut self) {
        self.bounds = self.inner.calc_bounds_2d()
    }
}

impl<T: CalcBounds2D + Transformed2D> Transformed2D for WithBounds2D<T> {
    fn transformed_2d(&self, mat: &Mat3) -> Self {
        let inner = self.inner.transformed_2d(mat);
        let bounds = inner.calc_bounds_2d();
        Self { inner, bounds }
    }
}

impl From<Geometry2D> for WithBounds2D<Geometry2D> {
    fn from(geo: Geometry2D) -> Self {
        Self::new(geo)
    }
}

#[test]
fn bounds_2d_test() {
    let bounds1 = Bounds2D::new(Vec2::new(0.0, 1.0), Vec2::new(2.0, 3.0));
    let bounds2 = Bounds2D::new(Vec2::new(4.0, 5.0), Vec2::new(6.0, 7.0));

    let bounds1 = bounds1.extend(bounds2);

    assert_eq!(bounds1.min, Vec2::new(0.0, 1.0));
    assert_eq!(bounds1.max, Vec2::new(6.0, 7.0));
}

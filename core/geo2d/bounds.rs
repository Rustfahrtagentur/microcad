// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry bounds.

use geo::{AffineOps, AffineTransform, coord};

use crate::{Scalar, Transformed2D, Vec2, geo2d::Rect, mat3_to_affine_transform};

/// 2D bounds, essentially an optional bounding rect.
#[derive(Debug, Default, Clone)]
pub struct Bounds2D(Option<Rect>);

impl Bounds2D {
    /// Create new 2D bounds.
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self(Some(Rect::new(
            coord! { x: min.x, y: min.y},
            coord! { x: max.x, y: max.y},
        )))
    }

    /// Minimum corner.
    pub fn min(&self) -> Option<Vec2> {
        self.0.as_ref().map(|s| Vec2::new(s.min().x, s.min().y))
    }

    /// Maximum corner.
    pub fn max(&self) -> Option<Vec2> {
        self.0.as_ref().map(|s| Vec2::new(s.max().x, s.max().y))
    }

    /// Return rect.
    pub fn rect(&self) -> &Option<Rect> {
        &self.0
    }

    /// Calculate extended bounds.
    pub fn extend(self, other: Bounds2D) -> Self {
        match (self.0, other.0) {
            (None, None) => Self(None),
            (None, Some(r)) | (Some(r), None) => Self(Some(r)),
            (Some(rect1), Some(rect2)) => Self::new(
                Vec2::new(
                    rect1.min().x.min(rect2.min().x),
                    rect1.min().y.min(rect2.min().y),
                ),
                Vec2::new(
                    rect1.max().x.max(rect2.max().x),
                    rect1.max().y.max(rect2.max().y),
                ),
            ),
        }
    }
}

impl AffineOps<Scalar> for Bounds2D {
    fn affine_transform(&self, transform: &AffineTransform<Scalar>) -> Self {
        Self(match &self.0 {
            Some(rect) => Some(rect.affine_transform(transform)),
            None => None,
        })
    }

    fn affine_transform_mut(&mut self, transform: &AffineTransform<Scalar>) {
        if let Some(rect) = &mut self.0 {
            rect.affine_transform_mut(transform)
        }
    }
}

impl Transformed2D for Bounds2D {
    fn transformed_2d(&self, _: &crate::RenderResolution, mat: &crate::Mat3) -> Self {
        self.affine_transform(&mat3_to_affine_transform(mat))
    }
}

impl From<Option<Rect>> for Bounds2D {
    fn from(rect: Option<Rect>) -> Self {
        match rect {
            Some(rect) => Self::new(rect.min().x_y().into(), rect.max().x_y().into()),
            None => Self(None),
        }
    }
}

/// Trait to return a bounding box of 2D geometry.
pub trait FetchBounds2D {
    /// Fetch bounds.
    fn fetch_bounds_2d(&self) -> Bounds2D;
}

#[test]
fn bounds_2d_test() {
    let bounds1 = Bounds2D::new(Vec2::new(0.0, 1.0), Vec2::new(2.0, 3.0));
    let bounds2 = Bounds2D::new(Vec2::new(4.0, 5.0), Vec2::new(6.0, 7.0));

    let bounds1 = bounds1.extend(bounds2);

    assert_eq!(bounds1.min(), Some(Vec2::new(0.0, 1.0)));
    assert_eq!(bounds1.max(), Some(Vec2::new(6.0, 7.0)));
}

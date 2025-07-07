// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Vec3;

/// Inner struct for bounds.
#[derive(Debug, Clone)]
struct BoundsInner {
    /// Minimum corner.
    min: Vec3,
    /// Maximum corner.
    max: Vec3,
}

/// A 3D bounds is a 3D bounding box with a minimum and maximum corner.
#[derive(Debug, Clone, Default)]
pub struct Bounds3D(Option<BoundsInner>);

impl Bounds3D {
    /// Create new 3D bounds.
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self(Some(BoundsInner { min, max }))
    }

    /// Minimum corner.
    pub fn min(&self) -> Option<Vec3> {
        self.0.as_ref().map(|s| s.min)
    }

    /// Maximum corner.
    pub fn max(&self) -> Option<Vec3> {
        self.0.as_ref().map(|s| s.max)
    }

    /// Calculate extended bounds.
    pub fn extend(self, other: Bounds3D) -> Self {
        match (self.0, other.0) {
            (None, None) => Self(None),
            (None, Some(b)) | (Some(b), None) => Self(Some(b)),
            (Some(b1), Some(b2)) => Self::new(
                Vec3::new(
                    b1.min.x.min(b2.min.x),
                    b1.min.y.min(b2.min.y),
                    b1.min.z.min(b2.min.z),
                ),
                Vec3::new(
                    b1.max.x.max(b2.max.x),
                    b1.max.y.max(b2.max.y),
                    b1.max.z.max(b2.max.z),
                ),
            ),
        }
    }
}

impl FromIterator<Vec3> for Bounds3D {
    fn from_iter<I: IntoIterator<Item = Vec3>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let first_point = match iter.next() {
            Some(point) => point,
            None => return Bounds3D(None),
        };

        let mut min = first_point;
        let mut max = first_point;

        iter.for_each(|p| {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);

            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        });

        Bounds3D::new(min, max)
    }
}

/// Trait to return a bounding box of 3D geometry.
pub trait FetchBounds3D {
    /// Fetch bounds.
    fn fetch_bounds_3d(&self) -> Bounds3D;
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use cgmath::ElementWise;

use crate::*;

/// Bounds3D type alias.
pub type Bounds3D = Bounds<Vec3>;

/// Corners iterator struct.
pub struct Bounds3DCorners {
    bounds: Bounds3D,
    index: u8, // Only goes from 0 to 7
}

impl Iterator for Bounds3DCorners {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 8 {
            return None;
        }

        let i = self.index;
        self.index += 1;

        let x = if i & 1 == 0 {
            self.bounds.min.x
        } else {
            self.bounds.max.x
        };
        let y = if i & 2 == 0 {
            self.bounds.min.y
        } else {
            self.bounds.max.y
        };
        let z = if i & 4 == 0 {
            self.bounds.min.z
        } else {
            self.bounds.max.z
        };

        Some(Vec3 { x, y, z })
    }
}

impl Bounds3D {
    /// Minimum and maximum corner.
    pub fn min_max(&self) -> (Vec3, Vec3) {
        (self.min, self.max)
    }

    /// Calculate extended bounds.
    pub fn extend(self, other: Bounds3D) -> Self {
        match (self.is_valid(), other.is_valid()) {
            (false, false) => Self::default(),
            (false, true) => other,
            (true, false) => self,
            (true, true) => Self::new(
                Vec3::new(
                    self.min.x.min(other.min.x),
                    self.min.y.min(other.min.y),
                    self.min.z.min(other.min.z),
                ),
                Vec3::new(
                    self.max.x.max(other.max.x),
                    self.max.y.max(other.max.y),
                    self.max.z.max(other.max.z),
                ),
            ),
        }
    }

    /// Check if bounds are valid
    pub fn is_valid(&self) -> bool {
        self.min.x <= self.max.x && self.min.y <= self.max.y && self.min.z <= self.max.z
    }

    /// Extend these bounds by point.
    pub fn extend_by_point(&mut self, p: Vec3) {
        self.min.x = p.x.min(self.min.x);
        self.min.y = p.y.min(self.min.y);
        self.min.z = p.z.min(self.min.z);
        self.max.x = p.x.max(self.max.x);
        self.max.y = p.y.max(self.max.y);
        self.max.z = p.z.max(self.max.z);
    }

    /// Corner iterator.
    pub fn corners(&self) -> Bounds3DCorners {
        Bounds3DCorners {
            bounds: self.clone(),
            index: 0,
        }
    }

    /// Maps a vec3 to bounds.
    ///
    /// The resulting `Vec3` is normalized between (0,0,0) = min  and (1,1,1) = max.
    pub fn map_vec3(&self, v: Vec3) -> Vec3 {
        (v - self.min).div_element_wise(self.max - self.min)
    }
}

impl Default for Bounds3D {
    fn default() -> Self {
        // Bounds are invalid by default.
        let min = Scalar::MAX;
        let max = Scalar::MIN;
        Self::new(Vec3::new(min, min, min), Vec3::new(max, max, max))
    }
}

impl FromIterator<Vec3> for Bounds3D {
    fn from_iter<I: IntoIterator<Item = Vec3>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        let first_point = match iter.next() {
            Some(point) => point,
            None => return Bounds3D::default(),
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

impl Transformed3D for Bounds3D {
    fn transformed_3d(&self, mat: &Mat4) -> Self {
        let mut bounds = Bounds3D::default();
        self.corners()
            .for_each(|corner| bounds.extend_by_point((mat * corner.extend(1.0)).truncate()));

        bounds
    }
}

/// Trait to return a bounding box of 3D geometry.
pub trait FetchBounds3D {
    /// Fetch bounds.
    fn fetch_bounds_3d(&self) -> Bounds3D;
}

/// Transformed version of a 3D geometry.
pub trait Transformed3D<T = Self> {
    /// Transform from matrix.
    fn transformed_3d(&self, mat: &Mat4) -> T;
}

/// Holds bounds for a 3D object.
#[derive(Clone, Default)]
pub struct WithBounds3D<T: FetchBounds3D> {
    /// Bounds.
    pub bounds: Bounds3D,
    /// The inner object.
    pub inner: T,
}

impl<T: FetchBounds3D> WithBounds3D<T> {
    /// Create a new object with bounds.
    pub fn new(inner: T) -> Self {
        Self {
            bounds: inner.fetch_bounds_3d(),
            inner,
        }
    }

    /// Update the bounds.
    pub fn update_bounds(&mut self) {
        self.bounds = self.inner.fetch_bounds_3d()
    }
}

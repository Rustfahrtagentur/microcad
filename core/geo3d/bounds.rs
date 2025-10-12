// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use cgmath::ElementWise;

use crate::{Mat4, Vec3};

/// Inner struct for bounds.
#[derive(Debug, Clone)]
struct BoundsInner {
    /// Minimum corner.
    min: Vec3,
    /// Maximum corner.
    max: Vec3,
}

/// Corners iterator struct.
pub struct Bounds3DCorners {
    bounds: BoundsInner,
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

    /// Minimum and maximum corner.
    pub fn min_max(&self) -> Option<(Vec3, Vec3)> {
        self.0.as_ref().map(|s| (s.min, s.max))
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

    /// Extend these bounds by point.
    pub fn extend_by_point(&mut self, p: Vec3) {
        match &mut self.0 {
            Some(bounds) => {
                *bounds = BoundsInner {
                    min: Vec3::new(
                        bounds.min.x.min(p.x),
                        bounds.min.y.min(p.y),
                        bounds.min.z.min(p.z),
                    ),
                    max: Vec3::new(
                        bounds.max.x.max(p.x),
                        bounds.max.y.max(p.y),
                        bounds.max.z.max(p.z),
                    ),
                }
            }
            None => *self = Self::new(p, p),
        }
    }

    /// Corner iterator.
    pub fn corners(&self) -> Bounds3DCorners {
        Bounds3DCorners {
            bounds: self.0.clone().expect("Bounds"),
            index: 0,
        }
    }

    /// Maps a vec3 to bounds.
    ///
    /// The resulting `Vec3` is normalized between (0,0,0) = min  and (1,1,1) = max.
    pub fn map_vec3(&self, v: Vec3) -> Vec3 {
        let min = self.min().unwrap();
        let max = self.max().unwrap();

        (v - min).div_element_wise(max - min)
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
    /// Bounds
    pub bounds: Bounds3D,
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

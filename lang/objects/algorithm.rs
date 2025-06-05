// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Algorithm

use crate::objects::*;
use microcad_core::*;

/// Algorithm trait.
pub trait Algorithm: std::fmt::Debug {
    /// Process this object nodes
    fn process(&self, node: ObjectNode);
}

impl Algorithm for BooleanOp {
    fn process(&self, _node: ObjectNode) {
        todo!("Implement boolean operations");
    }
}

/// Transformation matrix
#[derive(Clone, Debug, IntoStaticStr)]
pub enum AffineTransform {
    /// Translation
    Translation(Vec3),
    /// Rotation
    Rotation(Angle, Vec3),
    /// Scale
    Scale(Vec3),
    /// Uniform scale
    UniformScale(Scalar),
}

impl AffineTransform {
    /// Get the 2D transformation matrix
    pub fn mat2d(&self) -> Mat3 {
        match self {
            AffineTransform::Translation(v) => Mat3::from_translation(Vec2::new(v.x, v.y)),
            AffineTransform::Rotation(a, _) => Mat3::from_angle_z(*a),
            AffineTransform::Scale(v) => Mat3::from_nonuniform_scale(v.x, v.y),
            AffineTransform::UniformScale(s) => Mat3::from_scale(*s),
        }
    }

    /// Get the 3D transformation matrix
    pub fn mat3d(&self) -> Mat4 {
        match self {
            AffineTransform::Translation(v) => Mat4::from_translation(*v),
            AffineTransform::Rotation(a, v) => Mat3::from_axis_angle(*v, *a).into(),
            AffineTransform::Scale(v) => Mat4::from_nonuniform_scale(v.x, v.y, v.z),
            AffineTransform::UniformScale(s) => Mat4::from_scale(*s),
        }
    }
}

impl Algorithm for AffineTransform {
    fn process(&self, _node: ObjectNode) {
        todo!("Implement affine transforms")
    }
}

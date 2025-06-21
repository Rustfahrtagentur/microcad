// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Algorithm

use crate::model_tree::*;
use microcad_core::*;

/// Transformation trait.
pub trait Operation: std::fmt::Debug {
    /// The output type of this operation.
    ///
    /// By default, the output type is the same as the input node's output type.
    fn output_type(&self, node: ModelNode) -> ModelNodeOutputType {
        node.output_type()
    }

    /// Process the model
    fn process(&self, node: ModelNode);
}

impl Operation for BooleanOp {
    fn process(&self, _node: ModelNode) {
        todo!("Implement boolean operations");
    }
}

/// Transformation matrix
#[derive(Clone, Debug)]
pub enum AffineTransform {
    /// Translation.
    Translation(Vec3),
    /// Rotation around an axis.
    RotationAroundAxis(Angle, Vec3),
    /// Scale.
    Scale(Vec3),
    /// Uniform scale.
    UniformScale(Scalar),
}

impl AffineTransform {
    /// Get the 2D transformation matrix
    pub fn mat2d(&self) -> Mat3 {
        match self {
            AffineTransform::Translation(v) => Mat3::from_translation(Vec2::new(v.x, v.y)),
            AffineTransform::RotationAroundAxis(a, _) => Mat3::from_angle_z(*a),
            AffineTransform::Scale(v) => Mat3::from_nonuniform_scale(v.x, v.y),
            AffineTransform::UniformScale(s) => Mat3::from_scale(*s),
        }
    }

    /// Get the 3D transformation matrix
    pub fn mat3d(&self) -> Mat4 {
        match self {
            AffineTransform::Translation(v) => Mat4::from_translation(*v),
            AffineTransform::RotationAroundAxis(a, v) => Mat3::from_axis_angle(*v, *a).into(),
            AffineTransform::Scale(v) => Mat4::from_nonuniform_scale(v.x, v.y, v.z),
            AffineTransform::UniformScale(s) => Mat4::from_scale(*s),
        }
    }
}

impl Operation for AffineTransform {
    fn process(&self, _node: ModelNode) {
        todo!("Implement affine transforms")
    }
}

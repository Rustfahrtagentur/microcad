// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Algorithm

use crate::model::*;
use microcad_core::*;

/// Operation trait.
pub trait Operation: std::fmt::Debug {
    /// The output type of this operation.
    ///
    /// By default, the output type is undetermined.
    fn output_type(&self) -> OutputType {
        OutputType::NotDetermined
    }

    /// The input type of this operation.
    ///
    /// By default, the input type is undetermined.
    fn input_type(&self) -> OutputType {
        OutputType::NotDetermined
    }

    /// Process the model.
    fn process_2d(&self, _model: &Model) -> Geometries2D {
        unimplemented!()
    }

    /// Process the model.
    fn process_3d(&self, _model: &Model) -> Geometries3D {
        unimplemented!()
    }
}

impl Operation for BooleanOp {
    fn process_2d(&self, model: &Model) -> Geometries2D {
        let mut geometries = Geometries2D::default();

        if let Some(model) = model.into_inner_object_model() {
            let self_ = model.borrow();
            self_
                .children
                .iter()
                .for_each(|model| match &self_.element.value {
                    Element::Transform(affine_transform) => {
                        geometries.append(
                            model.process_2d(model).transformed_2d(
                                &self_.output.resolution,
                                &affine_transform.mat2d(),
                            ),
                        );
                    }
                    _ => {
                        geometries.append(model.process_2d(model));
                    }
                });
        }

        geometries.boolean_op(&model.borrow().output.resolution, self)
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

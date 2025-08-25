// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Algorithm

use crate::model::{render::RenderCache, *};
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
    fn process_2d(&self, _cache: &mut RenderCache, _model: &Model) -> Geometries2D {
        unimplemented!()
    }

    /// Process the model.
    fn process_3d(&self, _cache: &mut RenderCache, _model: &Model) -> Geometries3D {
        unimplemented!()
    }
}

/// Transformation matrix
#[derive(Clone, Debug)]
pub enum AffineTransform {
    /// Translation.
    Translation(Vec3),
    /// Generic rotation.
    Rotation(Mat3),
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
            AffineTransform::Rotation(m) => Mat3::from_cols(
                Vec3::new(m.x.x, m.x.y, 0.0),
                Vec3::new(m.y.x, m.y.y, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            AffineTransform::Scale(v) => Mat3::from_nonuniform_scale(v.x, v.y),
            AffineTransform::UniformScale(s) => Mat3::from_scale(*s),
        }
    }

    /// Get the 3D transformation matrix
    pub fn mat3d(&self) -> Mat4 {
        match self {
            AffineTransform::Translation(v) => Mat4::from_translation(*v),
            AffineTransform::Rotation(a) => Mat4::from_cols(
                a.x.extend(0.0),
                a.y.extend(0.0),
                a.z.extend(0.0),
                Vec3::new(0.0, 0.0, 0.0).extend(1.0),
            ),
            AffineTransform::Scale(v) => Mat4::from_nonuniform_scale(v.x, v.y, v.z),
            AffineTransform::UniformScale(s) => Mat4::from_scale(*s),
        }
    }
}

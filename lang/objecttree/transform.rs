// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::Algorithm;
use crate::*;
use microcad_core::*;
use strum::IntoStaticStr;

/// Transformation matrix
#[derive(Debug, Clone, IntoStaticStr)]
pub enum Transform {
    /// Translation
    Translation2D(Vec2),
    /// Rotation
    Rotation2D(Angle),
    /// Scale
    Scale2D(Scalar, Scalar),
    /// Affine transformation
    Affine2D(Mat3),

    /// Translation
    Translation3D(Vec3),
    /// Rotation
    Rotation3D(Angle, Scalar, Scalar, Scalar),
    /// Scale
    Scale3D(Scalar, Scalar, Scalar),
    /// Affine transformation
    Affine3D(Mat4),

    /// Uniform scale
    UniformScale(Scalar),
}

impl Transform {
    pub fn is_2d(&self) -> bool {
        matches!(
            self,
            Transform::Translation2D(_)
                | Transform::Rotation2D(_)
                | Transform::Scale2D(_, _)
                | Transform::Affine2D(_)
                | Transform::UniformScale(_)
        )
    }

    pub fn is_3d(&self) -> bool {
        matches!(
            self,
            Transform::Translation3D(_)
                | Transform::Rotation3D(_, _, _, _)
                | Transform::Scale3D(_, _, _)
                | Transform::Affine3D(_)
                | Transform::UniformScale(_)
        )
    }

    /// Get the 2D transformation matrix
    pub fn mat2d(&self) -> Mat3 {
        match self {
            Transform::Translation2D(v) => Mat3::from_translation(Vec2::new(v.x, v.y)),
            Transform::Rotation2D(a) => Mat3::from_angle_z(*a),
            Transform::Scale2D(x, y) => Mat3::from_nonuniform_scale(*x, *y),
            Transform::Affine2D(m) => *m,
            Transform::UniformScale(s) => Mat3::from_scale(*s),
            _ => panic!("Not a 2D transform"),
        }
    }

    /// Get the 3D transformation matrix
    pub fn mat3d(&self) -> Mat4 {
        if self.is_2d() {
            return self.mat2d().into();
        }

        match self {
            Transform::Translation3D(v) => Mat4::from_translation(*v),
            Transform::Rotation3D(a, x, y, z) => {
                Mat3::from_axis_angle(Vec3::new(*x, *y, *z), *a).into()
            }
            Transform::Scale3D(x, y, z) => Mat4::from_nonuniform_scale(*x, *y, *z),
            Transform::Affine3D(m) => *m,
            Transform::UniformScale(s) => Mat4::from_scale(*s),
            _ => panic!("Not a 3D transform"),
        }
    }
}

impl From<&Transform> for microcad_core::geo2d::Node {
    fn from(transform: &Transform) -> Self {
        microcad_core::geo2d::tree::transform(transform.mat2d())
    }
}

impl From<&Transform> for microcad_core::geo3d::Node {
    fn from(transform: &Transform) -> Self {
        microcad_core::geo3d::tree::transform(transform.mat3d())
    }
}

impl Algorithm for Transform {
    fn process_2d(&self, renderer: &mut Renderer2D, parent: ObjectNode) -> Result<geo2d::Node> {
        let geometries: Vec<_> = parent
            .children()
            .filter_map(|child| match &*child.borrow() {
                ObjectNodeInner::Group(_) => {
                    BooleanOp::Union.render_into_geometry2d(renderer, child.clone())
                }
                ObjectNodeInner::Primitive2D(renderable) => {
                    renderable.request_geometry(renderer).ok()
                }
                ObjectNodeInner::Transform(transform) => {
                    transform.render_into_geometry2d(renderer, child.clone())
                }
                ObjectNodeInner::Algorithm(algorithm) => {
                    algorithm.render_into_geometry2d(renderer, child.clone())
                }
                _ => None,
            })
            .collect();

        match geo2d::Geometry::boolean_op_multi(geometries, &BooleanOp::Union) {
            // If there are geometries, return the union of them and apply the transform
            Some(g) => Ok(geo2d::geometry(std::rc::Rc::new(
                g.as_ref().transform(self.mat2d()),
            ))),
            // If there are no geometries, return an empty group
            None => Ok(geo2d::group()),
        }
    }

    fn process_3d(&self, _renderer: &mut Renderer3D, _parent: ObjectNode) -> Result<geo3d::Node> {
        Ok(self.into())
    }
}

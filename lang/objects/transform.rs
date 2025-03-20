// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin transformations

use super::*;
use microcad_core::*;
use strum::IntoStaticStr;

/// Transformation matrix
#[derive(Debug, Clone, IntoStaticStr)]
pub enum Transform {
    /// Translation
    Translation(Vec3),
    /// Rotation
    Rotation(Angle, Vec3),
    /// Scale
    Scale(Vec3),

    /// Uniform scale
    UniformScale(Scalar),
}

impl Transform {
    /// Get the 2D transformation matrix
    pub fn mat2d(&self) -> Mat3 {
        match self {
            Transform::Translation(v) => Mat3::from_translation(Vec2::new(v.x, v.y)),
            Transform::Rotation(a, _) => Mat3::from_angle_z(*a),
            Transform::Scale(v) => Mat3::from_nonuniform_scale(v.x, v.y),
            Transform::UniformScale(s) => Mat3::from_scale(*s),
        }
    }

    /// Get the 3D transformation matrix
    pub fn mat3d(&self) -> Mat4 {
        match self {
            Transform::Translation(v) => Mat4::from_translation(*v),
            Transform::Rotation(a, v) => Mat3::from_axis_angle(*v, *a).into(),
            Transform::Scale(v) => Mat4::from_nonuniform_scale(v.x, v.y, v.z),
            Transform::UniformScale(s) => Mat4::from_scale(*s),
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
    fn process_2d(&self, renderer: &mut Renderer2D, parent: ObjectNode) -> CoreResult<geo2d::Node> {
        let geometries: Vec<_> = parent
            .children()
            .filter_map(|child| match &*child.borrow() {
                ObjectNodeInner::Group => {
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
            Some(g) => Ok(geo2d::geometry(Rc::new(g.transform(self.mat2d())))),
            // If there are no geometries, return an empty group
            None => Ok(geo2d::group()),
        }
    }

    fn process_3d(&self, renderer: &mut Renderer3D, parent: ObjectNode) -> CoreResult<geo3d::Node> {
        let geometries: Vec<_> = parent
            .children()
            .filter_map(|child| match &*child.borrow() {
                ObjectNodeInner::Group => {
                    BooleanOp::Union.render_into_geometry3d(renderer, child.clone())
                }
                ObjectNodeInner::Primitive3D(renderable) => {
                    renderable.request_geometry(renderer).ok()
                }
                ObjectNodeInner::Transform(transform) => {
                    transform.render_into_geometry3d(renderer, child.clone())
                }
                ObjectNodeInner::Algorithm(algorithm) => {
                    algorithm.render_into_geometry3d(renderer, child.clone())
                }
                _ => None,
            })
            .collect();

        match geo3d::Geometry::boolean_op_multi(geometries, &BooleanOp::Union) {
            // If there are geometries, return the union of them and apply the transform
            Some(g) => Ok(geo3d::geometry(Rc::new(g.transform(&self.mat3d())))),
            // If there are no geometries, return an empty group
            None => Ok(geo3d::group()),
        }
    }
}

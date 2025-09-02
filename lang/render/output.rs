// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model output types.

use std::rc::Rc;

use cgmath::SquareMatrix;

use microcad_core::{Geometry2D, Geometry3D, Mat3, Mat4, RenderResolution};

use crate::{
    model::{Model, OutputType},
    render::RenderResult,
};

/// Geometry 2D type alias.
pub type Geometry2DOutput = Option<Rc<Geometry2D>>;

/// Geometry 3D type alias.
pub type Geometry3DOutput = Option<Rc<Geometry3D>>;

/// The model output when a model has been processed.
#[derive(Debug, Clone)]
pub enum RenderOutput {
    /// 2D render output.
    Geometry2D {
        /// Local transformation matrix.
        local_matrix: Option<Mat3>,
        /// World transformation matrix.
        world_matrix: Option<Mat3>,
        /// The render resolution, calculated from transformation matrix.
        resolution: Option<RenderResolution>,
        /// The output geometry.
        geometry: Geometry2DOutput,
    },

    /// 3D render output.
    Geometry3D {
        /// Local transformation matrix.
        local_matrix: Option<Mat4>,
        /// World transformation matrix.
        world_matrix: Option<Mat4>,
        /// The render resolution, calculated from transformation matrix.
        resolution: Option<RenderResolution>,
        /// The output geometry.
        geometry: Geometry3DOutput,
    },
}

impl RenderOutput {
    /// Create new render output for model.
    pub fn new(model: &Model) -> RenderResult<Option<Self>> {
        let output_type = model.deduce_output_type();

        Ok(match output_type {
            OutputType::Geometry2D => {
                let local_matrix = model
                    .borrow()
                    .element
                    .get_affine_transform()?
                    .map(|affine_transform| affine_transform.mat2d())
                    .unwrap_or(Mat3::identity());

                Some(RenderOutput::Geometry2D {
                    local_matrix: Some(local_matrix),
                    world_matrix: None,
                    resolution: None,
                    geometry: None,
                })
            }

            OutputType::Geometry3D => {
                let local_matrix = model
                    .borrow()
                    .element
                    .get_affine_transform()?
                    .map(|affine_transform| affine_transform.mat3d())
                    .unwrap_or(Mat4::identity());

                Some(RenderOutput::Geometry3D {
                    local_matrix: Some(local_matrix),
                    world_matrix: None,
                    resolution: None,
                    geometry: None,
                })
            }
            _ => None,
        })
    }

    /// Set the world matrix for render output.
    pub fn set_world_matrix(&mut self, m: Mat4) {
        match self {
            RenderOutput::Geometry2D { world_matrix, .. } => *world_matrix = Some(mat4_to_mat3(&m)),
            RenderOutput::Geometry3D { world_matrix, .. } => {
                *world_matrix = Some(m);
            }
        }
    }

    /// Set the 2D geometry as render output.
    pub fn set_geometry_2d(&mut self, geo: Geometry2DOutput) {
        match self {
            RenderOutput::Geometry2D { geometry, .. } => *geometry = geo,
            RenderOutput::Geometry3D { .. } => unreachable!(),
        }
    }

    /// Get render resolution.
    pub fn resolution(&self) -> RenderResolution {
        match self {
            RenderOutput::Geometry2D { resolution, .. }
            | RenderOutput::Geometry3D { resolution, .. } => {
                resolution.as_ref().expect("Resolution").clone()
            }
        }
    }

    /// Set render resolution.
    pub fn set_resolution(&mut self, render_resolution: RenderResolution) {
        match self {
            RenderOutput::Geometry2D { resolution, .. }
            | RenderOutput::Geometry3D { resolution, .. } => *resolution = Some(render_resolution),
        }
    }

    /// Local matrix.
    pub fn local_matrix(&self) -> Option<Mat4> {
        match self {
            RenderOutput::Geometry2D { local_matrix, .. } => {
                local_matrix.as_ref().map(mat3_to_mat4)
            }
            RenderOutput::Geometry3D { local_matrix, .. } => *local_matrix,
        }
    }

    /// Get world matrix.
    pub fn world_matrix(&self) -> Mat4 {
        match self {
            RenderOutput::Geometry2D { world_matrix, .. } => {
                mat3_to_mat4(&world_matrix.expect("World matrix"))
            }
            RenderOutput::Geometry3D { world_matrix, .. } => world_matrix.expect("World matrix"),
        }
    }
}

fn mat4_to_mat3(m: &Mat4) -> Mat3 {
    Mat3::from_cols(m.x.truncate_n(2), m.y.truncate_n(2), m.w.truncate_n(2))
}

fn mat3_to_mat4(m: &Mat3) -> Mat4 {
    Mat4::new(
        m.x.x, m.x.y, 0.0, m.x.z, // First column: X basis + X translation
        m.y.x, m.y.y, 0.0, m.y.z, // Second column: Y basis + Y translation
        0.0, 0.0, 1.0, 0.0, // Z axis: identity (no change)
        0.0, 0.0, 0.0, 1.0, // Homogeneous row
    )
}

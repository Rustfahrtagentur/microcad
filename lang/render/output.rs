// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model output types.

use std::rc::Rc;

use microcad_core::{Geometry2D, Geometry3D, Mat3, Mat4, RenderResolution};

use crate::{model::*, render::*};

/// Geometry 2D type alias.
pub type Geometry2DOutput = Option<Rc<WithBounds2D<Geometry2D>>>;

/// Geometry 3D type alias.
pub type Geometry3DOutput = Option<Rc<WithBounds3D<Geometry3D>>>;

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
    pub fn new(model: &Model) -> RenderResult<Self> {
        let output_type = model.deduce_output_type();

        match output_type {
            OutputType::Geometry2D => {
                let local_matrix = model
                    .borrow()
                    .element
                    .get_affine_transform()?
                    .map(|affine_transform| affine_transform.mat2d());

                Ok(RenderOutput::Geometry2D {
                    local_matrix,
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
                    .map(|affine_transform| affine_transform.mat3d());

                Ok(RenderOutput::Geometry3D {
                    local_matrix,
                    world_matrix: None,
                    resolution: None,
                    geometry: None,
                })
            }
            output_type => Err(RenderError::InvalidOutputType(output_type)),
        }
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

    /// Set the 2D geometry as render output.
    pub fn set_geometry_3d(&mut self, geo: Geometry3DOutput) {
        match self {
            RenderOutput::Geometry2D { .. } => unreachable!(),
            RenderOutput::Geometry3D { geometry, .. } => *geometry = geo,
        }
    }

    /// Get render resolution.
    pub fn resolution(&self) -> &Option<RenderResolution> {
        match self {
            RenderOutput::Geometry2D { resolution, .. }
            | RenderOutput::Geometry3D { resolution, .. } => resolution,
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

impl std::fmt::Display for RenderOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            RenderOutput::Geometry2D {
                local_matrix,
                geometry,
                ..
            } => {
                write!(f, "2D: ")?;
                if local_matrix.is_none() && geometry.is_none() {
                    write!(f, "(nothing to render)")?;
                }
                if local_matrix.is_some() {
                    write!(f, "transform ")?;
                }
                if let Some(geometry) = geometry {
                    write!(
                        f,
                        "{} {}",
                        match &geometry.inner {
                            Geometry2D::Collection(geometries) =>
                                format!("Collection({} items)", geometries.len()),
                            geometry => geometry.name().to_string(),
                        },
                        geometry.bounds
                    )?;
                }
            }
            RenderOutput::Geometry3D {
                local_matrix,
                geometry,
                ..
            } => {
                write!(f, "3D: ")?;
                match (geometry, local_matrix) {
                    (None, None) => write!(f, "(nothing to render)"),
                    (None, Some(_)) => {
                        write!(f, "transform")
                    }
                    (Some(geometry), None) => write!(f, "{}", geometry.inner.name()),
                    (Some(geometry), Some(_)) => write!(f, "transformed {}", geometry.inner.name()),
                }?;
            }
        }

        if let Some(resolution) = self.resolution() {
            write!(f, " {resolution}")?
        }
        Ok(())
    }
}

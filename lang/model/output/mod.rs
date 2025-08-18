// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model output types.

mod geometry_output;
mod output_type;

pub use geometry_output::*;
pub use output_type::*;

use cgmath::SquareMatrix;
use microcad_core::{Geometries2D, Geometries3D, Mat3, Mat4, RenderResolution};

/// The model output when a model has been processed.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelOutput {
    /// The output geometry. Note: To be replaced by render geometry hash.
    pub geometry: GeometryOutput,
    /// World transformation matrix.
    pub world_matrix: Mat4,
    /// Local transformation matrix.
    pub local_matrix: Mat4,
    /// The render resolution, calculated from transformation matrix.
    pub resolution: RenderResolution,
}

impl ModelOutput {
    /// Create a new model output from model output type.
    pub fn new(ty: OutputType) -> Self {
        let geometry = match ty {
            OutputType::NotDetermined => GeometryOutput::None,
            OutputType::Geometry2D => GeometryOutput::Geometry2D(
                microcad_core::Geometry2D::Collection(Geometries2D::default()),
            ),
            OutputType::Geometry3D => GeometryOutput::Geometries3D(Geometries3D::default()),
            OutputType::InvalidMixed => GeometryOutput::Invalid,
        };

        Self {
            geometry,
            ..Default::default()
        }
    }

    /// Get model output type from geometry.
    pub fn output_type(&self) -> OutputType {
        self.geometry.model_output_type()
    }

    /// Local 2D transformation matrix (from 3D matrix).
    pub fn local_matrix_2d(&self) -> Mat3 {
        let m = &self.local_matrix;
        Mat3::from_cols(m.x.truncate_n(2), m.y.truncate_n(2), m.w.truncate_n(2))
    }

    /// World 2D transformation matrix (from 3D matrix).
    pub fn world_matrix_2d(&self) -> Mat3 {
        let m = &self.world_matrix;
        Mat3::from_cols(m.x.truncate_n(2), m.y.truncate_n(2), m.w.truncate_n(2))
    }
    /// Local 3D transformation matrix.
    pub fn local_matrix_3d(&self) -> Mat4 {
        self.local_matrix
    }

    /// World 3D transformation matrix.
    pub fn world_matrix_3d(&self) -> Mat4 {
        self.world_matrix
    }
}

impl Default for ModelOutput {
    fn default() -> Self {
        Self {
            geometry: Default::default(),
            world_matrix: microcad_core::Mat4::identity(),
            local_matrix: microcad_core::Mat4::identity(),
            resolution: Default::default(),
        }
    }
}

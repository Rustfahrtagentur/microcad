// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node output types.

mod geometry_output;
mod output_type;

pub use geometry_output::*;
pub use output_type::*;

use cgmath::SquareMatrix;
use microcad_core::{Geometries2D, Geometries3D, Mat3, Mat4, RenderResolution};

/// The model node output when a node has been processed.
#[derive(Debug, Clone)]
pub struct ModelNodeOutput {
    /// The output geometry.
    pub geometry: ModelNodeGeometryOutput,
    /// Transformation matrix.
    pub matrix: Mat4,
    /// The render resolution, calculated from transformation matrix.
    pub resolution: RenderResolution,
}

impl ModelNodeOutput {
    /// Create a new model node output from model output type.
    pub fn new(ty: ModelNodeOutputType) -> Self {
        let geometry = match ty {
            ModelNodeOutputType::NotDetermined => ModelNodeGeometryOutput::None,
            ModelNodeOutputType::Geometry2D => {
                ModelNodeGeometryOutput::Geometries2D(Geometries2D::default())
            }
            ModelNodeOutputType::Geometry3D => {
                ModelNodeGeometryOutput::Geometries3D(Geometries3D::default())
            }
            ModelNodeOutputType::InvalidMixed => ModelNodeGeometryOutput::Invalid,
        };

        Self {
            geometry,
            ..Default::default()
        }
    }

    /// Get model node output type from geometry.
    pub fn model_node_output_type(&self) -> ModelNodeOutputType {
        self.geometry.model_node_output_type()
    }

    /// 2D transformation matrix (from 3D matrix).
    pub fn matrix_2d(&self) -> Mat3 {
        let m = &self.matrix;
        Mat3::from_cols(m.x.truncate_n(2), m.y.truncate_n(2), m.w.truncate_n(2))
    }

    /// 3D transformation matrix.
    pub fn matrix_3d(&self) -> Mat4 {
        self.matrix
    }
}

impl Default for ModelNodeOutput {
    fn default() -> Self {
        Self {
            geometry: Default::default(),
            matrix: microcad_core::Mat4::identity(),
            resolution: Default::default(),
        }
    }
}

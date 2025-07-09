// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model node geometry output.

use microcad_core::{Geometries2D, Geometries3D};

use crate::model_tree::ModelNodeOutputType;

/// Geometry output of the model node.
#[derive(Debug, Default, Clone)]
pub enum ModelNodeGeometryOutput {
    /// No geometry output.
    #[default]
    None,
    /// 2d geometry.
    Geometries2D(Geometries2D),
    /// 3d geometry.
    Geometries3D(Geometries3D),
    /// Invalid geometry.
    Invalid,
}

impl ModelNodeGeometryOutput {
    /// Get output type from geometry output.
    pub(crate) fn model_node_output_type(&self) -> ModelNodeOutputType {
        match &self {
            Self::None => ModelNodeOutputType::NotDetermined,
            Self::Geometries2D(_) => ModelNodeOutputType::Geometry2D,
            Self::Geometries3D(_) => ModelNodeOutputType::Geometry3D,
            Self::Invalid => ModelNodeOutputType::InvalidMixed,
        }
    }
}

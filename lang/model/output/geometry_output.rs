// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Geometry model output.

use microcad_core::{Geometries2D, Geometries3D};

use crate::model::OutputType;

/// Geometry output of the model.
#[derive(
    Debug, Default, Clone, bincode::Encode, bincode::Decode, serde::Serialize, serde::Deserialize,
)]
pub enum GeometryOutput {
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

impl GeometryOutput {
    /// Get output type from geometry output.
    pub(crate) fn model_output_type(&self) -> OutputType {
        match &self {
            Self::None => OutputType::NotDetermined,
            Self::Geometries2D(_) => OutputType::Geometry2D,
            Self::Geometries3D(_) => OutputType::Geometry3D,
            Self::Invalid => OutputType::InvalidMixed,
        }
    }
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Geometry model output.

use std::rc::Rc;

use microcad_core::{Geometries3D, Geometry2D};

use crate::model::OutputType;

/// Geometry output of the model.
#[derive(Debug, Default, Clone)]
pub enum GeometryOutput {
    /// No geometry output.
    #[default]
    None,
    /// 2d geometry.
    Geometry2D(Option<Rc<Geometry2D>>),
    /// 3d geometry.
    Geometry3D(Geometries3D),
    /// Invalid geometry.
    Invalid,
}

impl GeometryOutput {
    /// Get output type from geometry output.
    pub(crate) fn model_output_type(&self) -> OutputType {
        match &self {
            Self::None => OutputType::NotDetermined,
            Self::Geometry2D(_) => OutputType::Geometry2D,
            Self::Geometry3D(_) => OutputType::Geometry3D,
            Self::Invalid => OutputType::InvalidMixed,
        }
    }
}

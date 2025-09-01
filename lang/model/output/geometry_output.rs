// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Geometry model output.

use std::rc::Rc;

use microcad_core::{Geometry2D, Geometry3D};

use crate::model::OutputType;

/// Geometry output of the model.
#[derive(Debug, Clone)]
pub enum GeometryOutput {
    /// 2d geometry.
    Geometry2D(Rc<Geometry2D>),
    /// 3d geometry.
    Geometry3D(Rc<Geometry3D>),
}

impl GeometryOutput {
    /// Get output type from geometry output.
    pub(crate) fn model_output_type(&self) -> OutputType {
        match &self {
            Self::Geometry2D(_) => OutputType::Geometry2D,
            Self::Geometry3D(_) => OutputType::Geometry3D,
        }
    }
}

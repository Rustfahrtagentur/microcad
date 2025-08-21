// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin operations.

use microcad_core::{BooleanOp, Geometries2D, Geometries3D};

use crate::model::*;

impl Operation for BooleanOp {
    fn process_2d(&self, model: &Model) -> Geometries2D {
        match model.into_group() {
            Some(model) => model.render_geometry_2d(),
            None => model.render_geometry_2d(),
        }
        .boolean_op(&model.borrow().output.resolution, self)
    }

    fn process_3d(&self, model: &Model) -> Geometries3D {
        match model.into_group() {
            Some(model) => model.render_geometry_3d(),
            None => model.render_geometry_3d(),
        }
        .boolean_op(&model.borrow().output.resolution, self)
    }
}

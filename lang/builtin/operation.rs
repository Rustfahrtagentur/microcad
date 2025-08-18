// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin operations.

use microcad_core::{BooleanOp, Geometry2D, Geometry3D};

use crate::model::*;

impl Operation for BooleanOp {
    fn process_2d(&self, model: &Model) -> Geometry2D {
        let geometry = match model.into_group() {
            Some(model) => model.render_geometry_2d(),
            None => model.render_geometry_2d(),
        };

        match geometry {
            Geometry2D::Collection(collection) => Geometry2D::Collection(
                collection.boolean_op(&model.borrow().output.resolution, self),
            ),
            geometry => geometry,
        }
    }

    fn process_3d(&self, model: &Model) -> Geometry3D {
        let geometry = match model.into_group() {
            Some(model) => model.render_geometry_3d(),
            None => model.render_geometry_3d(),
        };

        match geometry {
            Geometry3D::Collection(collection) => Geometry3D::Collection(
                collection.boolean_op(&model.borrow().output.resolution, self),
            ),
            geometry => geometry,
        }
    }
}

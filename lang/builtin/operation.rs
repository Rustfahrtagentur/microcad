// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin operations.

use microcad_core::{BooleanOp, Geometries3D, Geometry2D};

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

    fn process_3d(&self, model: &Model) -> Geometries3D {
        match model.into_group() {
            Some(model) => model.render_geometries_3d(),
            None => model.render_geometries_3d(),
        }
        .boolean_op(&model.borrow().output.resolution, self)
    }
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin operations.

use microcad_core::{BooleanOp, Geometries2D, Geometries3D, Transformed2D, Transformed3D};

use crate::model::*;

impl Operation for BooleanOp {
    fn process_2d(&self, model: &Model) -> Geometries2D {
        let mut geometries = Geometries2D::default();

        if let Some(model) = model.into_inner_object_model() {
            let self_ = model.borrow();
            self_.children.iter().for_each(|model| {
                let b = model.borrow();
                let mat = b.output.local_matrix_2d();
                geometries.append(
                    model
                        .process_2d(model)
                        .transformed_2d(&self_.output.resolution, &mat),
                );
            });
        }

        geometries.boolean_op(&model.borrow().output.resolution, self)
    }

    fn process_3d(&self, model: &Model) -> microcad_core::Geometries3D {
        let mut geometries = Geometries3D::default();

        if let Some(model) = model.into_inner_object_model() {
            let self_ = model.borrow();
            self_.children.iter().for_each(|model| {
                let b = model.borrow();
                let mat = b.output.local_matrix_3d();
                geometries.append(
                    model
                        .process_3d(model)
                        .transformed_3d(&self_.output.resolution, &mat),
                );
            });
        }

        geometries.boolean_op(&model.borrow().output.resolution, self)
    }
}

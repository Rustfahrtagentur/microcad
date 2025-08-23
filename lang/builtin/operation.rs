// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin operations.

use std::rc::Rc;

use microcad_core::{BooleanOp, Geometries3D, Geometry2D, Geometry3D};

use crate::model::{render::RenderCache, *};

impl Operation for BooleanOp {
    fn process_2d(&self, cache: &mut RenderCache, model: &Model) -> Rc<Geometry2D> {
        Rc::new(Geometry2D::MultiPolygon(
            match model.into_group() {
                Some(model) => model.render_geometry_2d(cache),
                None => model.render_geometry_2d(cache),
            }
            .boolean_op(&model.borrow().output.resolution, self),
        ))
    }

    fn process_3d(&self, cache: &mut RenderCache, model: &Model) -> Geometries3D {
        Geometries3D::new(vec![Geometry3D::Manifold(
            match model.into_group() {
                Some(model) => model.render_geometry_3d(cache),
                None => model.render_geometry_3d(cache),
            }
            .boolean_op(&model.borrow().output.resolution, self),
        )])
    }
}

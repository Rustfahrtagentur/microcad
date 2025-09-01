// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_core::*;
use microcad_lang::{
    eval::*,
    model::{
        render::{RenderCache, RenderResult},
        *,
    },
};

#[derive(Debug)]
pub struct Hull;

impl Operation for Hull {
    fn process_2d(&self, cache: &mut RenderCache, model: &Model) -> RenderResult<Rc<Geometry2D>> {
        let model_ = model.borrow();
        let output = model_.output.as_ref().expect("Some render output");

        Ok(Rc::new(Geometry2D::Polygon(
            model.render_geometry_2d(cache)?.hull(&output.resolution()),
        )))
    }

    fn process_3d(&self, cache: &mut RenderCache, model: &Model) -> RenderResult<Rc<Geometry3D>> {
        let model_ = model.borrow();
        let output = model_.output.as_ref().expect("Some render output");

        Ok(Rc::new(Geometry3D::Manifold(Rc::new(
            model.render_geometry_3d(cache)?.hull(&output.resolution()),
        ))))
    }
}

impl BuiltinWorkbenchDefinition for Hull {
    fn id() -> &'static str {
        "hull"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| Ok(BuiltinWorkpieceOutput::Operation(Box::new(Hull)))
    }
}

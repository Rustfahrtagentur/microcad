// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_core::*;
use microcad_lang::{
    eval::{BuiltinWorkbenchDefinition, BuiltinWorkpieceFn, BuiltinWorkpieceOutput},
    model::{
        render::{RenderCache, RenderResult},
        *,
    },
    syntax::*,
};

#[derive(Debug)]
pub struct Hull;

impl Operation for Hull {
    fn process_2d(&self, cache: &mut RenderCache, model: &Model) -> RenderResult<Rc<Geometry2D>> {
        Ok(Rc::new(Geometry2D::Polygon(
            model
                .render_geometry_2d(cache)?
                .hull(&model.borrow().output.resolution),
        )))
    }

    fn process_3d(&self, cache: &mut RenderCache, model: &Model) -> RenderResult<Rc<Geometry3D>> {
        Ok(Rc::new(Geometry3D::Manifold(Rc::new(
            model
                .render_geometry_3d(cache)?
                .hull(&model.borrow().output.resolution),
        ))))
    }
}

impl BuiltinWorkbenchDefinition for Hull {
    fn id() -> &'static str {
        "hull"
    }

    fn kind() -> WorkbenchKind {
        WorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| Ok(BuiltinWorkpieceOutput::Operation(Box::new(Hull)))
    }
}

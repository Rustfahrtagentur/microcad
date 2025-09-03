// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin hull operation.

use std::rc::Rc;

use microcad_lang::{builtin::*, render::*};

#[derive(Debug)]
pub struct Hull;

impl Operation for Hull {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model, resolution| {
            let model_ = model.borrow();
            let geometry: Geometry2DOutput = model_.children.render(context)?;
            Ok(geometry.map(|geometry| Rc::new(geometry.hull(&resolution))))
        })
    }

    fn process_3d(&self, _context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        todo!()
        /*let model_ = model.borrow();
        let output = model_.output.as_ref().expect("Some render output");

        Ok(Rc::new(Geometry3D::Manifold(Rc::new(
            model.render_geometry_3d(cache)?.hull(&output.resolution()),
        ))))*/
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

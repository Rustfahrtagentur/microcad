// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin center operation.

use std::rc::Rc;

use microcad_core::{Geometries2D, Geometry2D};
use microcad_lang::{builtin::*, render::*};

#[derive(Debug)]
pub struct Align;

impl Operation for Align {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model| {
            let model_ = model.borrow();
            let geometries: Geometries2D = model_.children.render_with_context(context)?;
            use microcad_core::traits::Align;
            Ok(Rc::new(Geometry2D::Collection(geometries).align().into()))
        })
    }

    fn process_3d(&self, _context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        todo!()
        /*context.update_3d(|context, model, resolution| {
            let geometry: Geometry3DOutput = model.render(context)?;
            geometry.map(|geometry| geometry.center(&resolution))
        })*/
    }
}

impl BuiltinWorkbenchDefinition for Align {
    fn id() -> &'static str {
        "align"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| Ok(BuiltinWorkpieceOutput::Operation(Box::new(Align)))
    }
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin center operation.

use std::rc::Rc;

use microcad_lang::{builtin::*, render::*};

#[derive(Debug)]
pub struct Center;

impl Operation for Center {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model, resolution| {
            let model_ = model.borrow();
            let geometry: Geometry2DOutput = model_.children.render(context)?;
            use microcad_core::traits::Center;
            Ok(geometry.map(|geometry| Rc::new(geometry.center(&resolution))))
        })
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        todo!()
        /*context.update_3d(|context, model, resolution| {
            let geometry: Geometry3DOutput = model.render(context)?;
            geometry.map(|geometry| geometry.center(&resolution))
        })*/
    }
}

impl BuiltinWorkbenchDefinition for Center {
    fn id() -> &'static str {
        "center"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| Ok(BuiltinWorkpieceOutput::Operation(Box::new(Center)))
    }
}

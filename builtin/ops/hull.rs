// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin hull operation.

use std::rc::Rc;

use microcad_lang::{builtin::*, render::*};

#[derive(Debug)]
pub struct Hull;

impl Operation for Hull {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model| {
            let model_ = model.borrow();
            let geometry: Geometry2DOutput = model_.children.render_with_context(context)?;

            Ok(Rc::new(geometry.inner.hull().into())) // TODO: Improve this API: model_.children.render_with_context(context)?.hull().into()
        })
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, model| {
            let model_ = model.borrow();
            let geometry: Geometry3DOutput = model_.children.render_with_context(context)?;

            Ok(Rc::new(geometry.inner.hull().into())) // TODO: Improve this API
        })
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

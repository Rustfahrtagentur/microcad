// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin operations.

use std::rc::Rc;

use microcad_core::{BooleanOp, Geometry2D};

use crate::{builtin::*, model::*, render::*, value::Tuple};

impl Operation for BooleanOp {
    fn process_2d(&self, context: &mut RenderContext) -> RenderResult<Geometry2DOutput> {
        context.update_2d(|context, model, resolution| {
            let geometry: Geometry2DOutput = model.render(context)?;
            match geometry {
                Some(output) => match output.as_ref() {
                    Geometry2D::Collection(geometries) => Ok(Some(Rc::new(
                        Geometry2D::MultiPolygon(geometries.boolean_op(&resolution, self)),
                    ))),
                    output => Ok(Some(Rc::new(output.clone()))),
                },
                output => Ok(output),
            }
        })
    }

    fn process_3d(&self, _context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        todo!()
        /*Ok(Rc::new(Geometry3D::Manifold(
            match model.into_group() {
                Some(model) => model.prerender(cache),
                None => model.prerender(cache),
            }?
            .boolean_op(&model.borrow().resolution(), self),
        )))*/
    }
}

/// Union operation.
pub struct Union;

impl BuiltinWorkbenchDefinition for Union {
    fn id() -> &'static str {
        "union"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| {
            Ok(BuiltinWorkpieceOutput::Operation(Box::new(
                BooleanOp::Union,
            )))
        }
    }
}

/// Difference operation.
pub struct Subtract;

impl BuiltinWorkbenchDefinition for Subtract {
    fn id() -> &'static str {
        "subtract"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| {
            Ok(BuiltinWorkpieceOutput::Operation(Box::new(
                BooleanOp::Subtract,
            )))
        }
    }
}

/// Intersection operation.
pub struct Intersect;

impl BuiltinWorkbenchDefinition for Intersect {
    fn id() -> &'static str {
        "intersect"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| {
            Ok(BuiltinWorkpieceOutput::Operation(Box::new(
                BooleanOp::Intersect,
            )))
        }
    }
}

impl From<BooleanOp> for BuiltinWorkpiece {
    fn from(value: BooleanOp) -> Self {
        match value {
            BooleanOp::Union => Union::workpiece(Creator::new(Union::symbol(), Tuple::default())),
            BooleanOp::Subtract => {
                Subtract::workpiece(Creator::new(Subtract::symbol(), Tuple::default()))
            }
            BooleanOp::Intersect => {
                Intersect::workpiece(Creator::new(Intersect::symbol(), Tuple::default()))
            }
            BooleanOp::Complement => unimplemented!(),
        }
    }
}

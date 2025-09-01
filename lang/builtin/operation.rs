// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin operations.

use std::rc::Rc;

use microcad_core::{BooleanOp, Geometry2D, Geometry3D};

use crate::{
    eval::*,
    model::{
        render::{RenderCache, RenderResult},
        *,
    },
    value::Tuple,
};

impl Operation for BooleanOp {
    fn process_2d(&self, cache: &mut RenderCache, model: &Model) -> RenderResult<Rc<Geometry2D>> {
        Ok(Rc::new(Geometry2D::MultiPolygon(
            match model.into_group() {
                Some(model) => model.render_geometry_2d(cache),
                None => model.render_geometry_2d(cache),
            }?
            .boolean_op(&model.borrow().resolution(), self),
        )))
    }

    fn process_3d(&self, cache: &mut RenderCache, model: &Model) -> RenderResult<Rc<Geometry3D>> {
        Ok(Rc::new(Geometry3D::Manifold(
            match model.into_group() {
                Some(model) => model.render_geometry_3d(cache),
                None => model.render_geometry_3d(cache),
            }?
            .boolean_op(&model.borrow().resolution(), self),
        )))
    }
}

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

pub struct Difference;

impl BuiltinWorkbenchDefinition for Difference {
    fn id() -> &'static str {
        "difference"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| {
            Ok(BuiltinWorkpieceOutput::Operation(Box::new(
                BooleanOp::Difference,
            )))
        }
    }
}

pub struct Intersection;

impl BuiltinWorkbenchDefinition for Intersection {
    fn id() -> &'static str {
        "intersection"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|_| {
            Ok(BuiltinWorkpieceOutput::Operation(Box::new(
                BooleanOp::Intersection,
            )))
        }
    }
}

impl From<BooleanOp> for BuiltinWorkpiece {
    fn from(value: BooleanOp) -> Self {
        match value {
            BooleanOp::Union => Union::workpiece(&Tuple::default()),
            BooleanOp::Difference => Difference::workpiece(&Tuple::default()),
            BooleanOp::Intersection => Intersection::workpiece(&Tuple::default()),
            BooleanOp::Complement => unimplemented!(),
        }
    }
}

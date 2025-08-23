// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_core::*;
use microcad_lang::{
    model::{render::RenderCache, *},
    resolve::*,
    syntax::*,
};

#[derive(Debug)]
struct Hull;

impl Operation for Hull {
    fn process_2d(&self, cache: &mut RenderCache, model: &Model) -> Rc<Geometry2D> {
        Rc::new(Geometry2D::Polygon(
            model
                .render_geometry_2d(cache)
                .hull(&model.borrow().output.resolution),
        ))
    }

    fn process_3d(&self, cache: &mut RenderCache, model: &Model) -> Rc<Geometry3D> {
        Rc::new(Geometry3D::Manifold(Rc::new(
            model
                .render_geometry_3d(cache)
                .hull(&model.borrow().output.resolution),
        )))
    }
}

/// Creates a symbol containing a hull operation.
pub fn hull() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("hull"), None, &|_, _, _| {
        Ok(ModelBuilder::new_operation(Hull).build().into())
    })
}

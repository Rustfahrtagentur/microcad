// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{model::*, resolve::*, syntax::*};

#[derive(Debug)]
struct Hull;

impl Operation for Hull {
    fn process_2d(&self, model: &Model) -> Geometries2D {
        model
            .render_geometries_2d()
            .hull(&model.borrow().output.resolution)
    }

    fn process_3d(&self, _node: &Model) -> Geometries3D {
        std::todo!("Hull operation for 3D")
    }
}

/// Creates a symbol containing a hull operation.
pub fn hull() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("hull"), None, &|_, _, _| {
        Ok(ModelBuilder::new_operation(Hull).build().into())
    })
}

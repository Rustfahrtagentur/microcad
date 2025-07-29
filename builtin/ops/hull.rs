// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{model::*, resolve::*, src_ref::*, syntax::*};

#[derive(Debug)]
struct Hull;

impl Operation for Hull {
    fn process_2d(&self, model: &Model) -> microcad_core::Geometries2D {
        let mut geometries = Geometries2D::default();

        if let Some(model) = model.reach_into_group() {
            let self_ = model.borrow();

            self_.children.iter().for_each(|model| {
                let b = model.borrow();
                let mat = b.output.local_matrix_2d();
                geometries.append(
                    model
                        .process_2d(model)
                        .transformed_2d(&b.output.resolution, &mat),
                );
            });
        }

        geometries.hull(&model.borrow().output.resolution)
    }

    fn process_3d(&self, _node: &microcad_lang::model::Model) -> microcad_core::Geometries3D {
        std::todo!("Hull operation for 3D")
    }
}

/// Creates a symbol containing a difference operation.
pub fn hull() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("hull"), None, &|_, _, _| {
        Ok(ModelBuilder::new_operation(Hull, SrcRef(None))
            .build()
            .into())
    })
}

// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{model::*, resolve::*, src_ref::*, syntax::*};

#[derive(Debug)]
struct Extrude;

impl Operation for Extrude {
    fn process_3d(&self, _node: &microcad_lang::model::Model) -> microcad_core::Geometries3D {
        unimplemented!("Extrude")
    }
}

/// Creates a symbol containing a difference operation.
pub fn extrude() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("extrude"), None, &|_, _, _| {
        Ok(ModelBuilder::new_operation(Extrude, SrcRef(None))
            .build()
            .into())
    })
}

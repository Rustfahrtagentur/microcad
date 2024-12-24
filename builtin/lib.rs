// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Standard library

mod assert;
mod context_builder;
mod export;
mod math;
mod namespace_builder;
mod print;
mod transform;

/// Algorithm module, e.g. `std::algorithm::difference`
pub mod algorithm;
/// Module containing builtin 2D geometries like `circle` or `rect`
pub mod geo2d;
/// Module containing builtin 3D geometries like `sphere` or `cube`
#[cfg(feature = "geo3d")]
pub mod geo3d;

#[cfg(test)]
mod tests;

use microcad_lang::{builtin_module, eval::*, parse::*, sym::*};

pub use context_builder::ContextBuilder;
pub use export::export;

use microcad_core::ExportSettings;
use namespace_builder::NamespaceBuilder;

/// Build the standard module
pub fn builtin_module() -> ParseResult<std::rc::Rc<NamespaceDefinition>> {
    Ok(NamespaceBuilder::new("__builtin")
        // TODO: is this correct= Shouldn't this use add_builtin_module() =
        .add(math::builtin_module()?.into())
        .add(geo2d::builtin_module().into())
        .add(geo3d::builtin_module().into())
        .add(algorithm::builtin_module().into())
        .add(transform::builtin_namespace().into())
        .add(assert::builtin_fn().into())
        .add(print::builtin_fn().into())
        .add(
            builtin_module!(export(filename: String) {
                let export_settings = ExportSettings::with_filename(filename.clone());

                Ok(microcad_export::export(export_settings))
            })
            .into(),
        )
        .build())
}

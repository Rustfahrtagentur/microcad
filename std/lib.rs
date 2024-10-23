// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#![warn(missing_docs)]

//! µCAD Standard library

/// Algorithm module, e.g. `std::algorithm::difference`
pub mod algorithm;

mod context_builder;
mod export;

/// Module containing builtin 2D geometries like `circle` or `rect`
pub mod geo2d;
mod math;
mod namespace_builder;

/// Module containing builtin 3D geometries like `sphere` or `cube`
#[cfg(feature = "geo3d")]
pub mod geo3d;

#[cfg(test)]
mod tests;

use microcad_lang::parameter;
use microcad_lang::parameter_list;
use microcad_lang::src_ref::SrcReferrer;
use microcad_lang::{builtin_module, eval::*, function_signature, parse::*};

pub use context_builder::ContextBuilder;
pub use export::export;

use microcad_core::ExportSettings;
use namespace_builder::NamespaceBuilder;

/// Build the standard module
pub fn builtin_module() -> std::rc::Rc<NamespaceDefinition> {
    NamespaceBuilder::new("__builtin")
        // TODO: is this correct= Shouldn't this use add_builtin_module() =
        .add(math::builtin_module().into())
        .add(geo2d::builtin_module().into())
        .add(geo3d::builtin_module().into())
        .add(algorithm::builtin_module().into())
        .add(
            BuiltinFunction::new(
                "assert".into(),
                function_signature!(parameter_list![
                    parameter!(condition: Bool),
                    parameter!(message: String = "Assertion failed")
                ]),
                &|args, ctx| {
                    let message: String = args["message"].clone().try_into()?;
                    let condition: bool = args["condition"].clone().try_into()?;
                    if !condition {
                        use anyhow::anyhow;
                        use microcad_lang::diag::PushDiag;

                        if let Some(condition_src) =
                            ctx.get_source_string(args["condition"].src_ref())
                        {
                            ctx.error(args.src_ref(), anyhow!("{message}: {condition_src}",))?;
                        } else {
                            ctx.error(args.src_ref(), anyhow!("{message}"))?;
                        }
                    }
                    Ok(None)
                },
            )
            .into(),
        )
        .add(
            builtin_module!(export(filename: String) {
                let export_settings = ExportSettings::with_filename(filename.clone());

                Ok(microcad_export::export(export_settings))
            })
            .into(),
        )
        .build()
}

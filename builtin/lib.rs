// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library

mod assert;
pub mod context_builder;
mod geo2d;
mod geo3d;
pub mod import;
mod math;
mod ops;
mod print;
mod transform;

/// Global test initialization.
#[cfg(test)]
#[ctor::ctor]
fn init() {
    env_logger::init();
}

pub use microcad_lang::builtin::*;
use microcad_lang::{eval::Stdout, resolve::*};

pub(crate) use assert::*;
pub use context_builder::*;
pub(crate) use math::*;
pub(crate) use ops::*;
pub(crate) use print::*;
pub(crate) use transform::*;

/// Build the standard module
pub fn builtin_module() -> Symbol {
    ModuleBuilder::new("__builtin".try_into().expect("unexpected name error"))
        .symbol(assert())
        .symbol(assert_eq())
        .symbol(assert_valid())
        .symbol(assert_invalid())
        .symbol(type_of())
        .symbol(print())
        .symbol(error())
        .symbol(warning())
        .symbol(info())
        .symbol(ops())
        .symbol(transform())
        .symbol(math())
        .symbol(import::import())
        .symbol(geo2d::geo2d())
        .symbol(geo3d::geo3d())
        .build()
}

/// Get built-in importers.
pub fn builtin_importers() -> ImporterRegistry {
    ImporterRegistry::default().insert(microcad_import::toml::TomlImporter)
}

/// Get built-in exporters.
pub fn builtin_exporters() -> ExporterRegistry {
    ExporterRegistry::new().insert(microcad_export::svg::SvgExporter)
}

/// Built-in context.
pub fn builtin_context(
    root: Symbol,
    search_paths: &[std::path::PathBuf],
) -> microcad_lang::eval::Context {
    ContextBuilder::new(root, builtin_module(), search_paths, Box::new(Stdout))
        .importers(builtin_importers())
        .exporters(builtin_exporters())
        .build()
}

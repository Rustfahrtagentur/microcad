// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library

mod debug;
mod geo2d;
mod geo3d;
pub mod import;
mod log;
mod math;
mod ops;
mod print;

/// Global test initialization.
#[cfg(test)]
#[ctor::ctor]
fn init() {
    env_logger::init();
}

use std::str::FromStr;

pub use microcad_lang::builtin::*;
use microcad_lang::{diag::*, eval::*, ty::Ty, value::*};

/// Return type of argument.
fn type_of() -> Symbol {
    let id = Identifier::from_str("type_of").expect("valid id");
    Symbol::new_builtin(id, None, &|_, args, _| {
        if let Ok(arg) = args.get_single() {
            let ty = arg.1.ty();
            return Ok(Value::String(ty.to_string()));
        }
        Ok(Value::None)
    })
}

/// Return the count of elements in an array or string.
fn count() -> Symbol {
    Symbol::new_builtin(Identifier::no_ref("count"), None, &|_params, args, ctx| {
        let arg = args.get_single()?;
        Ok(match &arg.1.value {
            Value::String(s) => Value::Integer(s.chars().count() as i64),
            Value::Array(a) => Value::Integer(a.len() as i64),
            _ => {
                ctx.error(arg.1, EvalError::BuiltinError("Value has no count.".into()))?;
                Value::None
            }
        })
    })
}

/// Build the standard module
pub fn builtin_module() -> Symbol {
    ModuleBuilder::new("__builtin".try_into().expect("unexpected name error"))
        .symbol(debug::debug())
        .symbol(log::log())
        .symbol(count())
        .symbol(type_of())
        .symbol(print::print())
        .symbol(ops::ops())
        .symbol(math::math())
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
    ExporterRegistry::new()
        .insert(microcad_export::svg::SvgExporter)
        .insert(microcad_export::stl::StlExporter)
        .insert(microcad_export::json::JsonExporter)
}

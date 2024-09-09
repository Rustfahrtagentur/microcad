#![warn(missing_docs)]

//! µCAD Standard library

mod algorithm;
mod context_builder;
mod export;
mod geo2d;
mod math;
mod namespace_builder;

#[cfg(feature = "geo3d")]
mod geo3d;

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
pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    NamespaceBuilder::new("std")
        // TODO: is this correct= Shouldn't this use add_builtin_module() =
        .add_module(math::builtin_module())
        .add_module(geo2d::builtin_module())
        .add_module(geo3d::builtin_module())
        .add_module(algorithm::builtin_module())
        .add_builtin_function(BuiltinFunction::new(
            "assert".into(),
            function_signature!(parameter_list![
                parameter!(condition: Bool),
                parameter!(message: String = "Assertion failed")
            ]),
            &|args, ctx| {
                let message: String = args["message"].clone().try_into()?;
                let condition: bool = args["condition"].clone().try_into()?;
                if !condition {
                    use microcad_lang::diagnostics::AddDiagnostic;
                    ctx.error(args.src_ref(), format!("Assertion failed: {message}"));
                    Err(EvalError::AssertionFailed(message))
                } else {
                    Ok(None)
                }
            },
        ))
        .add_builtin_module(builtin_module!(export(filename: String) {
            let export_settings = ExportSettings::with_filename(filename.clone());

            Ok(microcad_core::export::export(export_settings))
        }))
        .build()
}

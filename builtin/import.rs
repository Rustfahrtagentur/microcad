// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library

use microcad_lang::{
    builtin::ImporterRegistryAccess,
    diag::PushDiag,
    eval::{ArgumentMap, ArgumentMatch},
    parameter,
    syntax::Identifier,
    value::Value,
};

use crate::Symbol;

pub fn import() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("import"),
        Some(
            vec![
                parameter!(filename: String),
                parameter!(id: String = String::new()),
            ]
            .into(),
        ),
        &|parameter_values, argument_values, context| match ArgumentMap::find_match(
            argument_values,
            parameter_values.expect("Parameter values"),
        ) {
            Ok(arg_map) => context.import(&arg_map),
            Err(err) => {
                context.error(argument_values, err)?;
                Ok(Value::None)
            }
        },
    )
}

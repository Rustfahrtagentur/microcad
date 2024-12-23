// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{eval::*, function_signature, parameter, parameter_list};

pub fn builtin_fn() -> BuiltinFunction {
    BuiltinFunction::new(
        "print".into(),
        function_signature!(parameter_list![parameter!(message: String)]),
        &|args, _| {
            let message: String = args["message"].clone().try_into()?;
            println!("{message}");
            Ok(None)
        },
    )
}

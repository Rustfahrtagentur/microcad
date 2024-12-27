// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin print method

use std::sync::Mutex;

use lazy_static::lazy_static;
use microcad_lang::{eval::*, function_signature, parameter, parameter_list};

lazy_static! {
    /// Alternate print output buffer
    pub static ref output: Mutex<String> = Mutex::new(String::new());
}

/// Create print function
pub fn builtin_fn() -> BuiltinFunction {
    output.lock().expect("sync error").clear();
    BuiltinFunction::new(
        "print".into(),
        function_signature!(parameter_list![parameter!(message: String)]),
        &|args, _| {
            let message: String = args["message"].clone().try_into()?;
            // print on terminal...
            println!("{message}");
            // ..and write into output buffer
            output.lock().expect("sync error").push_str(&message);
            Ok(None)
        },
    )
}

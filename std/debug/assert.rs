// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{
    eval::BuiltinFunction,
    parameter_list,
    parse::{Expression, Identifier, Parameter},
    r#type::Type,
};

pub fn builtin_fn() -> BuiltinFunction {
    BuiltinFunction::new(
        "assert".into(),
        microcad_lang::parse::function::FunctionSignature::new(
            parameter_list![
                Parameter {
                    name: Identifier::builtin("condition"),
                    specified_type: Some(Type::Bool),
                    ..Default::default()
                },
                Parameter {
                    name: Identifier::builtin("message"),
                    specified_type: Some(Type::String),
                    default_value: Some(
                        Expression::literal_from_str("Assertion failed").expect("Invalid literal")
                    )
                }
            ],
            None,
        ),
        &|args, _| {
            let message: String = args.get_value("message");
            let condition: bool = args.get_value("condition");
            assert!(condition, "{message}");
            Ok(None)
        },
    )
}


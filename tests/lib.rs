// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod use_test;

#[cfg(test)]
use log::debug;

#[cfg(test)]
use microcad_lang::{parser::*, resolve::*, syntax::*};

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));
/*
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_source_file_test.rs"));
*/
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_markdown_test.rs"));

#[cfg(test)]
fn evaluate_file(filename: &str) -> microcad_lang::eval::EvalContext {
    use microcad_lang::eval::*;

    let filename = format!("../tests/test_cases/{filename}");
    EvalContext::from_source(&filename, microcad_builtin::builtin_namespace(), &[])
        .expect(&filename)
}

#[test]
fn namespaces() {
    assert!(evaluate_file("syntax/namespace.µcad").eval().is_ok());
}

#[test]
fn scopes() {
    assert!(evaluate_file("../tests/test_cases/syntax/scopes.µcad")
        .eval()
        .is_ok());
}

#[test]
fn context_with_symbols() {
    let mut context = evaluate_file("../tests/test_cases/syntax/call.µcad");

    context
        .lookup(
            &"__builtin::assert_valid"
                .try_into()
                .expect("unexpected name error"),
        )
        .expect("symbol not found");
    context
        .lookup(
            &"__builtin::assert_invalid"
                .try_into()
                .expect("unexpected name error"),
        )
        .expect("symbol not found");

    assert!(context.eval().is_ok());
}

#[test]
fn module_implicit_init() {
    microcad_lang::env_logger_init();

    let mut context = evaluate_file("syntax/module/implicit_init.µcad");
    debug!("Source File:\n{}", context.get_root());

    if let Ok(node) =
        context.lookup(&Identifier(microcad_lang::src_ref::Refer::none("a".into())).into())
    {
        let id = node.borrow().id();
        assert_eq!(id, "a");

        if let SymbolDefinition::Module(module_definition) = &node.borrow().def {
            use microcad_lang::eval::CallTrait;
            let _value = module_definition
                .call(
                    &Parser::parse_rule::<CallArgumentList>(
                        microcad_lang::parser::Rule::call_argument_list,
                        "b = 3.0",
                        0,
                    )
                    .expect("Valid CallArgumentList"),
                    &mut context,
                )
                .expect("Valid value");
        } else {
            panic!("Symbol is not a module")
        }

        debug!("{}", id);
    } else {
        panic!("Symbol not found");
    }
}

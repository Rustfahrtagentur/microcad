// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));
/*
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_source_file_test.rs"));
*/
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_markdown_test.rs"));

#[cfg(test)]
static TEST_OUT_DIR: &str = "output";

#[cfg(test)]
static DEFAULT_TEST_FILE: &str = "../tests/test_cases/algorithm/difference.µcad";

/// Assure `TEST_OUT_DIR` exists
#[cfg(test)]
fn make_test_out_dir() -> std::path::PathBuf {
    let test_out_dir = std::path::PathBuf::from(TEST_OUT_DIR);
    if !test_out_dir.exists() {
        std::fs::create_dir_all(&test_out_dir).expect("test error");
    }
    test_out_dir
}

#[cfg(test)]
fn load_source_file(
    filename: &str,
) -> (
    std::rc::Rc<microcad_lang::syntax::SourceFile>,
    microcad_lang::eval::EvalContext,
) {
    use microcad_builtin::*;
    use microcad_lang::{eval::*, syntax::*};
    let source_file = SourceFile::load(format!("../tests/test_cases/{filename}"))
        .expect("cannot load test file: {filename}");

    let mut context = EvalContext::from_source_file(source_file.clone(), vec![]);
    context.add_symbol(builtin_namespace());

    assert!(source_file.eval(&mut context).is_ok());

    (source_file, context)
}

#[test]
fn namespaces() {
    use microcad_lang::eval::*;

    let (source_file, mut context) = load_source_file("syntax/namespace.µcad");

    //println!("{}", symbol_node.borrow());

    assert!(source_file.eval(&mut context).is_ok());
}

#[test]
fn scopes() {
    use microcad_builtin::*;
    use microcad_lang::{eval::*, syntax::*};

    let source_file =
        SourceFile::load("../tests/test_cases/syntax/scopes.µcad").expect("cannot load test file");

    let mut context = EvalContext::from_source_file(source_file.clone(), vec![]);
    context.add_symbol(builtin_namespace());

    assert!(source_file.eval(&mut context).is_ok());
}

#[test]
fn context_with_symbols() {
    use microcad_builtin::*;
    use microcad_lang::{eval::*, syntax::*};
    let source_file =
        SourceFile::load("../tests/test_cases/syntax/call.µcad").expect("cannot load test file");
    let mut context = EvalContext::from_source_file(source_file.clone(), vec![]);

    context.add_symbol(builtin_namespace());
    context
        .fetch_symbol(
            &"__builtin::assert_valid"
                .try_into()
                .expect("unexpected name error"),
        )
        .expect("symbol not found");
    context
        .fetch_symbol(
            &"__builtin::assert_invalid"
                .try_into()
                .expect("unexpected name error"),
        )
        .expect("symbol not found");
    assert!(source_file.eval(&mut context).is_ok());
}

#[test]
fn module_implicit_init() {
    let (source_file, context) = load_source_file("syntax/module/implicit_init.µcad");
}

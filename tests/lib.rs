// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::{
    eval::{Eval, EvalContext},
    parse::SourceFile,
};

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

// Assure `TEST_OUT_DIR` exists
#[cfg(test)]
fn make_test_out_dir() -> std::path::PathBuf {
    let test_out_dir = std::path::PathBuf::from(TEST_OUT_DIR);
    if !test_out_dir.exists() {
        std::fs::create_dir_all(&test_out_dir).expect("test error");
    }
    test_out_dir
}

#[test]
fn namespaces() {
    let source_file = SourceFile::load("../tests/test_cases/syntax/namespace.µcad").unwrap();

    use microcad_lang::resolve::Resolve;

    let symbol_node = source_file.resolve(None);

    println!("{}", symbol_node.borrow());

    let mut context = EvalContext::from_source_file(source_file.clone());

    let node = source_file.eval(&mut context);
}

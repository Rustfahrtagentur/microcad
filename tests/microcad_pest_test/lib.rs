// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Generate tests out of grammar.pest file which include µcad code fragments

mod pest_file;
mod pest_result;
mod rust_writer;

pub use pest_file::*;
pub use pest_result::*;
pub use rust_writer::*;

/// Single test
#[derive(Debug, Clone)]
pub struct PestTest {
    source: String,
    result: PestResult,
    line: Option<usize>,
}

/// Generates tests from Pest grammar file
pub fn generate(
    parser_struct_name: &str,
    rule_enum_name: &str,
    grammar_file: impl AsRef<std::path::Path>,
) {
    use std::{env::*, fs::*, path::*};

    let out_dir = var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("microcad_pest_test.rs");

    PestFile::from_file(&grammar_file)
        .unwrap()
        .generate_test_rs(
            parser_struct_name,
            rule_enum_name,
            &mut File::create(dest_path).unwrap(),
        )
        .unwrap();
    println!("cargo:rerun-if-changed={}", grammar_file.as_ref().display());
}

//pub fn generate_test_case_from_file(test_file: impl AsRef<std::path::Path>)

#[test]
fn test_comment() {
    env_logger::init();

    let test = r#"//`test`: ok # Test"#;
    let test = test.parse::<PestTest>().unwrap();
    assert_eq!(test.source, "test");
    assert_eq!(test.result, PestResult::Ok("Test".into()));
}

#[test]
fn parse_pest_file() {
    env_logger::init();

    let test = r#"
            //`test1`: ok # Ok Test
            //`test2`: error # Error Test
            expr = {  "{" ~ expr_interior ~ "}" }
        "#;

    let test = test.parse::<PestFile>().unwrap();
    assert_eq!(test.len(), 1);
    assert_eq!(test[0].name, "expr");
    assert_eq!(test[0].tests.len(), 2);
    assert_eq!(test[0].tests[0].source, "test1");
    assert_eq!(test[0].tests[0].result, PestResult::Ok("Ok Test".into()));
    assert_eq!(test[0].tests[1].source, "test2");
    assert_eq!(
        test[0].tests[1].result,
        PestResult::Err("Error Test".into())
    );
}

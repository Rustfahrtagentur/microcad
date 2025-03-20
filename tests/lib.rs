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
    use microcad_lang::*;
    let source_file = SourceFile::load("../tests/test_cases/syntax/namespace.µcad").expect("");
    let symbol_node = source_file.resolve(None);

    println!("{}", symbol_node.borrow());

    let mut context = EvalContext::from_source_file(source_file.clone());
    let _ = source_file.eval(&mut context);
}

#[test]
fn scopes() {
    use microcad_lang::*;
    let source_file = SourceFile::load("../tests/test_cases/syntax/scopes.µcad").expect("");

    let mut context = EvalContext::from_source_file(source_file.clone());

    let _ = source_file.eval(&mut context);
}

#[test]
fn context_with_symbols() {
    use microcad_lang::*;
    let source_file = SourceFile::load("../tests/test_cases/syntax/namespace.µcad").expect("");
    let mut context = EvalContext::from_source_file(source_file.clone());

    let builtin_namespace = NamespaceDefinition::new("__builtin".into());
    let mut builtin_symbol = SymbolNode::new(SymbolDefinition::Namespace(builtin_namespace), None);

    let assert_valid = BuiltinFunction::new("assert_valid".into(), &|args, ctx| {
        println!("assert valid called");
        Ok(Value::Invalid)
    });
    let assert_invalid = BuiltinFunction::new("assert_invalid".into(), &|args, ctx| {
        println!("assert invalid called");
        Ok(Value::Invalid)
    });

    SymbolNode::insert_child(
        &mut builtin_symbol,
        SymbolNode::new(SymbolDefinition::BuiltinFunction(assert_valid), None),
    );
    SymbolNode::insert_child(
        &mut builtin_symbol,
        SymbolNode::new(SymbolDefinition::BuiltinFunction(assert_invalid), None),
    );

    println!("{}", builtin_symbol.borrow());

    context.add_symbol(builtin_symbol);
}

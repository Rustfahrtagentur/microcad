// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Global test initialization.
#[cfg(test)]
#[ctor::ctor]
fn init() {
    env_logger::init();
}

mod context;
mod part;
mod source_file_test;

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_source_file_test.rs"));

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_markdown_test.rs"));

/// Evaluate the context for a file in the `test_cases` folder.
#[cfg(test)]
fn context_for_file(filename: &str) -> microcad_lang::eval::Context {
    use microcad_lang::eval::*;

    let filename = format!("../tests/test_cases/{filename}");
    Context::from_source(
        &filename,
        microcad_builtin::builtin_module(),
        &["../lib".into()],
    )
    .expect(&filename)
}

/// Test a single source file.
///
/// See [`source_file_test::SourceFileTest::test`] for more info.
#[cfg(test)]
fn test_source_file(filename: &str) {
    source_file_test::SourceFileTest::new(filename).test();
}

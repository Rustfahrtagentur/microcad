// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD pest test

#![warn(missing_docs, clippy::unwrap_used)]

/// pest test main
fn main() {
    microcad_pest_test::generate(
        "microcad_lang::parser::Parser",
        "microcad_lang::parser::Rule",
        "../lang/grammar.pest",
    );

    if let Err(err) = microcad_markdown_test::generate("..") {
        panic!("error generating rust test code from markdown file: {err}");
    }

    if let Err(err) = microcad_source_file_test::generate("test_cases") {
        panic!("error generating rust test code from source: {err}");
    }
}

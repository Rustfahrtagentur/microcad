// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Useful macros

/// Generates a test for some µcad code
#[macro_export]
macro_rules! microcad_test {
    ($code:literal) => {
        let source = match microcad_lang::syntax::SourceFile::load_from_str($code) {
            Err(err) => panic!("Parse error:\n{err}"),
            Ok(source) => source,
        };
        let symbols = source.resolve(None);
        let mut context = microcad_lang::eval::Context::new(
            symbols.clone(),
            microcad_builtin::builtin_namespace(),
            &["../lib".into()],
            Box::new(microcad_lang::eval::Stdout),
        );
        match context.eval() {
            Err(_) => panic!("Evaluation error(s):\n{context}"),
            Ok(_) => (),
        }
    };
}

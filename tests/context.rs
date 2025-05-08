// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::eval::*;

#[test]
fn use_statements() {
    let mut context = Context::from_source(
        "test_cases/context/use_test.µcad",
        microcad_builtin::builtin_namespace(),
        &["../lib".into()],
    )
    .expect("context");
    context.eval().expect("successful evaluation");
}

#[test]
fn locals() {
    let mut context = Context::from_source(
        "test_cases/context/locals.µcad",
        microcad_builtin::builtin_namespace(),
        &["../lib".into()],
    )
    .expect("context");
    context.eval().expect("successful evaluation");
}

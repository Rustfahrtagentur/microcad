// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[test]
fn context_with_symbols() {
    use microcad_lang::eval::Lookup;
    let mut context = crate::context_for_file("syntax/call.µcad");

    context
        .lookup(
            &"__builtin::debug::assert_valid"
                .try_into()
                .expect("unexpected name error"),
        )
        .expect("symbol not found");
    context
        .lookup(
            &"__builtin::debug::assert_invalid"
                .try_into()
                .expect("unexpected name error"),
        )
        .expect("symbol not found");

    assert!(context.eval().is_ok());
}

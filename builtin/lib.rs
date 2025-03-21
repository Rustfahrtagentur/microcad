// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library

mod algorithm;
mod assert;
mod print;

use microcad_lang::*;

/// Build the standard module
pub fn builtin_namespace() -> RcMut<SymbolNode> {
    let mut builtin_namespace = SymbolNode::new_builtin_namespace("__builtin");
    assert::build(&mut builtin_namespace);
    print::build(&mut builtin_namespace);
    algorithm::build(&mut builtin_namespace);
    builtin_namespace
}

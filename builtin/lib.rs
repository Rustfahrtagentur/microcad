// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library

mod algorithm;
mod assert;
mod print;

use microcad_lang::*;

/// Build the standard module
pub fn builtin_module() -> RcMut<SymbolNode> {
    let mut builtin_namespace = SymbolNode::new_builtin_namespace("__builtin");
    assert::build(&mut builtin_namespace);
    print::build(&mut builtin_namespace);
    SymbolNode::insert_child(&mut builtin_namespace, algorithm::build());
    builtin_namespace
}

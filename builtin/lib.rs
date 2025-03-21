// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad builtin library

mod assert;
mod print;

use microcad_lang::*;

/// Build the standard module
pub fn builtin_module() -> RcMut<SymbolNode> {
    let builtin_namespace = NamespaceDefinition::new("__builtin".into());
    let mut builtin_symbol = SymbolNode::new(SymbolDefinition::Namespace(builtin_namespace), None);

    assert::build(&mut builtin_symbol);
    print::build(&mut builtin_symbol);

    builtin_symbol
}

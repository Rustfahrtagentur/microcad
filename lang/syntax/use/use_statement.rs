// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement syntax element

use crate::{
    resolve::{Symbol, SymbolMap},
    src_ref::*,
    syntax::*,
};

/// Use statement:
///
/// ```ucad
///
/// use std::*;
/// ```
#[derive(Clone, Debug)]
pub struct UseStatement {
    /// export of use
    pub visibility: Visibility,
    /// Use declaration
    pub decl: UseDeclaration,
    /// source code reference
    pub src_ref: SrcRef,
}

impl UseStatement {
    /// Resolve use statement to multiple symbols
    pub fn resolve(&self, parent: Option<Symbol>) -> SymbolMap {
        match self.visibility {
            // Private symbols are processed later in `Context::use_symbol`
            Visibility::Private => SymbolMap::new(),
            // Public symbols are put into resolving symbol map
            Visibility::Public => {
                let mut symbols = SymbolMap::new();
                let (id, symbol) = self.decl.resolve(parent.clone());
                symbols.insert(id, symbol);
                symbols
            }
        }
    }
}

impl SrcReferrer for UseStatement {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.visibility {
            Visibility::Private => write!(f, "use ")?,
            Visibility::Public => write!(f, "pub use ")?,
        }
        write!(f, "{}", self.decl)?;
        Ok(())
    }
}

impl PrintSyntax for UseStatement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}UseStatement", "")?;
        self.decl.print_syntax(f, depth + 1)
    }
}

// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement syntax element.

use crate::{src_ref::*, syntax::*};
use strum::IntoStaticStr;

/// Use declaration.
///
/// A use declaration is an element of a use statement.
/// It can be a single symbol, all symbols from a module, or an alias.
///
/// ```ucad
/// use std::print;
/// use std::*;
/// use std::print as p;
/// ```
///
#[derive(Clone, Debug, IntoStaticStr)]
pub enum UseDeclaration {
    /// Import symbols given as qualified names: `use a, b`
    Use(QualifiedName),
    /// Import all symbols from a module: `use std::*`
    UseAll(QualifiedName),
    /// Import as alias: `use a as b`
    UseAlias(QualifiedName, Identifier),
}

impl SrcReferrer for UseDeclaration {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Use(name) => name.src_ref(),
            Self::UseAll(name) => name.src_ref(),
            Self::UseAlias(name, _) => name.src_ref(),
        }
    }
}

impl std::fmt::Display for UseDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UseDeclaration::Use(name) => write!(f, "{name}"),
            UseDeclaration::UseAll(name) => write!(f, "{name}::*"),
            UseDeclaration::UseAlias(name, alias) => write!(f, "{name} as {alias}"),
        }
    }
}

impl TreeDisplay for UseDeclaration {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        // use declaration is transparent
        match self {
            UseDeclaration::Use(name) => writeln!(f, "{:depth$}Use {name}", ""),
            UseDeclaration::UseAll(name) => writeln!(f, "{:depth$}Use {name}::*", ""),
            UseDeclaration::UseAlias(name, alias) => {
                writeln!(f, "{:depth$}Use {name} as {alias}", "")
            }
        }
    }
}

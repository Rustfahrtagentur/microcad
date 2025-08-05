// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement syntax element

use crate::{resolve::*, src_ref::*, syntax::*};
use strum::IntoStaticStr;

/// Use declaration
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
#[derive(Clone, Debug, IntoStaticStr, serde::Serialize, serde::Deserialize)]
pub enum UseDeclaration {
    /// Import symbols given as qualified names: `use a, b`
    Use(QualifiedName),
    /// Import all symbols from a module: `use std::*`
    UseAll(QualifiedName),
    /// Import as alias: `use a as b`
    UseAlias(QualifiedName, Identifier),
}

impl UseDeclaration {
    /// resolve public use declaration (shall not be called with private use statements)
    pub fn resolve(&self, parent: Option<Symbol>) -> Option<(Identifier, Symbol)> {
        match self {
            UseDeclaration::Use(name) => {
                let identifier = name.last().expect("Identifier");
                Some((
                    identifier.clone(),
                    Symbol::new(
                        SymbolDefinition::Alias(identifier.clone(), name.clone()),
                        parent,
                    ),
                ))
            }
            UseDeclaration::UseAll(_) => None,
            UseDeclaration::UseAlias(name, alias) => Some((
                alias.clone(),
                Symbol::new(SymbolDefinition::Alias(alias.clone(), name.clone()), parent),
            )),
        }
    }
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

impl PrintSyntax for UseDeclaration {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        match self {
            UseDeclaration::Use(name) => writeln!(f, "{:depth$}Use {name}", ""),
            UseDeclaration::UseAll(name) => writeln!(f, "{:depth$}Use {name}::*", ""),
            UseDeclaration::UseAlias(name, alias) => {
                writeln!(f, "{:depth$}Use {name} as {alias}", "")
            }
        }
    }
}

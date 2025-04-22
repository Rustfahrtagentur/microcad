// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement syntax element

use crate::{resolve::*, src_ref::*, syntax::*, Id};
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
#[derive(Clone, Debug, IntoStaticStr)]
pub enum UseDeclaration {
    /// Import symbols given as qualified names: `use a, b`
    Use(QualifiedName, SrcRef),
    /// Import all symbols from a module: `use std::*`
    UseAll(QualifiedName, SrcRef),
    /// Import as alias: `use a as b`
    UseAlias(QualifiedName, Identifier, SrcRef),
}

impl UseDeclaration {
    /// resolve public use declaration (shall not be called with private use statements)
    pub fn resolve(&self, parent: Option<SymbolNodeRcMut>) -> (Id, SymbolNodeRcMut) {
        match self {
            UseDeclaration::Use(qualified_name, _) => {
                let identifier = qualified_name.last().expect("Identifier");
                (
                    identifier.id().clone(),
                    SymbolNodeRcMut::new(SymbolNode {
                        def: SymbolDefinition::Alias(identifier.clone(), qualified_name.clone()),
                        parent,
                        children: Default::default(),
                    }),
                )
            }
            UseDeclaration::UseAll(_qualified_name, _src_ref) => todo!(),
            UseDeclaration::UseAlias(qualified_name, identifier, _) => (
                identifier.id().clone(),
                SymbolNodeRcMut::new(SymbolNode {
                    def: SymbolDefinition::Alias(identifier.clone(), qualified_name.clone()),
                    parent,
                    children: Default::default(),
                }),
            ),
        }
    }
}

impl SrcReferrer for UseDeclaration {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Use(_, src_ref) => src_ref.clone(),
            Self::UseAll(_, src_ref) => src_ref.clone(),
            Self::UseAlias(_, _, src_ref) => src_ref.clone(),
        }
    }
}

impl std::fmt::Display for UseDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UseDeclaration::Use(name, _) => write!(f, "{name}"),
            UseDeclaration::UseAll(name, _) => write!(f, "{name}::*"),
            UseDeclaration::UseAlias(name, alias, _) => write!(f, "{name} as {alias}"),
        }
    }
}

impl PrintSyntax for UseDeclaration {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        match self {
            UseDeclaration::Use(name, _) => writeln!(f, "{:depth$}Use {name}", ""),
            UseDeclaration::UseAll(name, _) => writeln!(f, "{:depth$}Use {name}::*", ""),
            UseDeclaration::UseAlias(name, alias, _) => {
                writeln!(f, "{:depth$}Use {name} as {alias}", "")
            }
        }
    }
}

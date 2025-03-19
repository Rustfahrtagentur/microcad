// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement parser entity

use crate::{parse::*, parser::*, src_ref::*};

/// Use statement:
///
/// ```ucad
///
/// use std::*;
/// ```
#[derive(Clone, Debug)]
pub struct UseStatement {
    visibility: Visibility,
    decls: Vec<UseDeclaration>,
    src_ref: SrcRef,
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
        for (i, decl) in self.decls.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{decl}")?;
        }
        Ok(())
    }
}

impl Parse for UseStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::use_statement);

        let mut visibility = Visibility::default();
        let mut decls = Vec::new();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::use_declaration => {
                    decls.push(UseDeclaration::parse(pair)?);
                }
                Rule::visibility => {
                    visibility = Visibility::parse(pair)?;
                }
                _ => unreachable!("Invalid use declaration"),
            }
        }

        Ok(Self {
            visibility,
            decls,
            src_ref: pair.into(),
        })
    }
}

impl PrintSyntax for UseStatement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}UseStatement", "")?;
        self.decls
            .iter()
            .try_for_each(|d| d.print_syntax(f, depth + 1))
    }
}

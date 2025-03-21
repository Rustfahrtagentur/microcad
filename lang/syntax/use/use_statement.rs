// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement syntax element

use crate::{src_ref::*, syntax::*};

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
    /// Use declarations
    pub decls: Vec<UseDeclaration>,
    /// source code reference
    pub src_ref: SrcRef,
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

impl PrintSyntax for UseStatement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}UseStatement", "")?;
        self.decls
            .iter()
            .try_for_each(|d| d.print_syntax(f, depth + 1))
    }
}

// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Use statement parser entity

use crate::{eval::*, parse::*, parser::*, src_ref::*, sym::*};

/// Use statement:
///
/// ```ucad
///
/// use std::*;
/// ```
#[derive(Clone, Debug)]
pub struct UseStatement(Visibility, Vec<UseDeclaration>, SrcRef);

impl SrcReferrer for UseStatement {
    fn src_ref(&self) -> SrcRef {
        self.2.clone()
    }
}

impl std::fmt::Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.0 {
            Visibility::Private => write!(f, "use ")?,
            Visibility::Public => write!(f, "pub use ")?,
        }
        for (i, decl) in self.1.iter().enumerate() {
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

        Ok(Self(visibility, decls, pair.into()))
    }
}

impl Eval for UseStatement {
    type Output = Option<SymbolTable>;

    fn eval(&self, context: &mut EvalContext) -> EvalResult<Self::Output> {
        // Return a symbol table if the use statement is public
        match self.0 {
            Visibility::Public => {
                let mut symbols = SymbolTable::default();
                for decl in &self.1 {
                    let mut symbol_table = decl.eval(context)?;
                    for (name, symbol) in symbol_table.iter() {
                        use crate::sym::Symbols;
                        context.add_alias(symbol.as_ref().clone(), name.clone());
                    }

                    symbols.merge(&mut symbol_table);
                }
                Ok(Some(symbols))
            }
            Visibility::Private => {
                for decl in &self.1 {
                    let symbol_table = decl.eval(context)?;
                    for (name, symbol) in symbol_table.iter() {
                        use crate::sym::Symbols;
                        context.add_alias(symbol.as_ref().clone(), name.clone());
                    }
                }
                Ok(None)
            }
        }
    }
}

// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module body parser entity

use crate::{
    parse::*,
    parser::*,
    resolve::{SymbolMap, SymbolNodeRc},
    src_ref::*,
};

/// Module definition body
///
/// An example for a module definition body:
///
/// ```microCAD
/// module donut {
///     a = 2; // Pre-init statement
///
///     init(d: length) { // init definition
///         radius = d / 2;
///     }
///
///     init(r: length) { // Another init definition
///
///     }
///
///     b = 2; // Post-init statement
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Body {
    /// Module statements before init
    pub statements: Vec<Statement>,
    /// Source code reference
    src_ref: SrcRef,
}

impl Body {
    pub fn fetch_symbol_map_from(
        statements: &Vec<Statement>,
        parent: Option<SymbolNodeRc>,
    ) -> SymbolMap {
        let mut symbol_map = SymbolMap::default();
        use crate::resolve::Resolve;
        if let Some(parent) = &parent {
            symbol_map.insert("super".into(), parent.clone());
        }

        // Iterate over all statement fetch definitions
        for statement in statements {
            match statement {
                Statement::Module(m) => {
                    symbol_map.insert(
                        m.name.id().clone(),
                        std::rc::Rc::new(m.resolve(parent.clone())),
                    );
                }
                Statement::Namespace(n) => {
                    symbol_map.insert(
                        n.name.id().clone(),
                        std::rc::Rc::new(n.resolve(parent.clone())),
                    );
                }
                Statement::Function(f) => {
                    symbol_map.insert(
                        f.name.id().clone(),
                        std::rc::Rc::new(f.resolve(parent.clone())),
                    );
                }
                _ => {}
            }
        }

        symbol_map
    }

    pub fn fetch_symbol_map(&self, parent: Option<SymbolNodeRc>) -> SymbolMap {
        Self::fetch_symbol_map_from(&self.statements, parent)
    }
}

impl SrcReferrer for Body {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for Body {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::body);
        let mut body = Self::default();
        for pair in pair.inner() {
            if pair.as_rule() == Rule::statement {
                body.statements.push(Statement::parse(pair.clone())?)
            }
        }
        body.src_ref = pair.into();

        Ok(body)
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, " {{")?;

        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }

        writeln!(f, "}}")?;
        Ok(())
    }
}

impl PrintSyntax for Body {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}Body:", "")?;
        self.statements
            .iter()
            .try_for_each(|s| s.print_syntax(f, depth + 1))
    }
}

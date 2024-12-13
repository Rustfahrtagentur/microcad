// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Namespace statement parser entities

use crate::{eval::*, parse::*, parser::*, src_ref::*};

/// Namespace statement
#[derive(Debug, Clone)]
pub enum NamespaceStatement {
    /// Use statement
    Use(UseStatement),
    /// Module definition
    ModuleDefinition(std::rc::Rc<ModuleDefinition>),
    /// Namespace definition
    NamespaceDefinition(std::rc::Rc<NamespaceDefinition>),
    /// Function definition
    FunctionDefinition(std::rc::Rc<FunctionDefinition>),
    /// Assignment
    Assignment(Assignment),
}

impl SrcReferrer for NamespaceStatement {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Use(us) => us.src_ref(),
            Self::ModuleDefinition(md) => md.src_ref(),
            Self::NamespaceDefinition(nd) => nd.src_ref(),
            Self::FunctionDefinition(fd) => fd.src_ref(),
            Self::Assignment(a) => a.src_ref(),
        }
    }
}

impl Parse for NamespaceStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::namespace_statement);
        let first = pair.inner().next().unwrap();
        Ok(match first.as_rule() {
            Rule::use_statement => Self::Use(UseStatement::parse(first)?),
            Rule::module_definition => {
                Self::ModuleDefinition(std::rc::Rc::<ModuleDefinition>::parse(first)?)
            }
            Rule::namespace_definition => {
                Self::NamespaceDefinition(std::rc::Rc::<NamespaceDefinition>::parse(first)?)
            }
            Rule::function_definition => {
                Self::FunctionDefinition(std::rc::Rc::<FunctionDefinition>::parse(first)?)
            }
            Rule::assignment => Self::Assignment(Assignment::parse(first)?),
            rule => unreachable!(
                "Unexpected namespace statement, got {:?} {:?}",
                rule,
                first.clone()
            ),
        })
    }
}

impl Eval for NamespaceStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> std::result::Result<Self::Output, EvalError> {
        match self {
            Self::Use(use_statement) => {
                if let Some(symbols) = use_statement.eval(context)? {
                    for (id, symbol) in symbols.iter() {
                        context.add_alias(symbol.as_ref().clone(), id.clone());
                    }
                }
            }
            Self::Assignment(assignment) => {
                assignment.eval(context)?;
            }
            Self::FunctionDefinition(function_definition) => {
                context.add(function_definition.clone().into());
            }
            Self::ModuleDefinition(module_definition) => {
                context.add(module_definition.clone().into());
            }
            Self::NamespaceDefinition(namespace_definition) => {
                let namespace_definition = namespace_definition.eval(context)?;
                context.add(namespace_definition);
            }
        }

        Ok(())
    }
}

// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD statement

use crate::{eval::*, parse::*, parse::*, parser::*, src_ref::*};

/// µCAD source file statement
#[derive(Clone, Debug)]
pub enum Statement {
    /// Use statement, e.g. `use std::*;`
    Use(UseStatement),
    /// Module definition, e.g. `module foo(r: scalar) { info("Hello, world, {r}!"); }`
    ModuleDefinition(std::rc::Rc<ModuleDefinition>),
    /// Namespace definition, e.g. `namespace foo { }`
    NamespaceDefinition(std::rc::Rc<NamespaceDefinition>),
    /// Function definition, e.g. `fn foo() { }`
    FunctionDefinition(std::rc::Rc<FunctionDefinition>),
    /// Assignment statement, e.g. `a = 10;`
    Assignment(Assignment),
    /// For loop, e.g. `for i in 0..10 { }`
    For(ForStatement),
    /// Expression statement, e.g. `a + b;`
    Expression(Expression),
}

impl SrcReferrer for Statement {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Use(u) => u.src_ref(),
            Self::ModuleDefinition(m) => m.src_ref(),
            Self::NamespaceDefinition(n) => n.src_ref(),
            Self::FunctionDefinition(f) => f.src_ref(),
            Self::Assignment(a) => a.src_ref(),
            Self::For(f) => f.src_ref(),
            Self::Expression(e) => e.src_ref(),
        }
    }
}

impl Parse for Statement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::source_file_statement);
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
            Rule::module_for_statement => Self::For(ForStatement::parse(first)?),
            Rule::expression | Rule::expression_no_semicolon => {
                Self::Expression(Expression::parse(first)?)
            }
            rule => unreachable!(
                "Unexpected source file statement, got {:?} {:?}",
                rule,
                first.clone()
            ),
        })
    }
}

impl Eval for Statement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        match self {
            Self::Use(use_statement) => {
                use_statement.eval(context)?;
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
                let namespace_symbol = namespace_definition.eval(context)?;
                context.add(namespace_symbol);
            }
            Self::For(for_statement) => {
                for_statement.eval(context)?;
            }
            Self::Expression(expression) => {
                expression.eval(context)?;
            }
        }
        Ok(())
    }
}

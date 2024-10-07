// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Node body parser entity

use microcad_render::Node;

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Module initialization statement
#[derive(Clone, Debug)]
pub enum NodeBodyStatement {
    /// Use statement
    Use(UseStatement),
    /// Expresson
    Expression(Expression),
    /// Assignment
    Assignment(Assignment),
}

impl SrcReferrer for NodeBodyStatement {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::Use(us) => us.src_ref(),
            Self::Expression(expression) => expression.src_ref(),
            Self::Assignment(assignment) => assignment.src_ref(),
        }
    }
}

impl Parse for NodeBodyStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let first = pair.inner().next().unwrap();
        Ok(match first.as_rule() {
            Rule::use_statement => NodeBodyStatement::Use(UseStatement::parse(first)?),
            Rule::expression => NodeBodyStatement::Expression(Expression::parse(first)?),
            Rule::assignment => NodeBodyStatement::Assignment(Assignment::parse(first)?),
            _ => unreachable!(),
        })
    }
}

impl Eval for NodeBodyStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> std::result::Result<Self::Output, EvalError> {
        match self {
            Self::Use(use_statement) => use_statement.eval(context),
            Self::Expression(expression) => {
                let value = expression.eval(context)?;
                match value {
                    Value::Node(_) => Ok(()),
                    _ => {
                        //use crate::diagnostics::AddDiagnostic;
                        // TODO Expression results should be Option<Value>
                        //context.error(expression, format!("Expected node, got {}", value.ty()));
                        Ok(())
                    }
                }
            }
            Self::Assignment(assignment) => assignment.eval(context),
        }
    }
}

impl std::fmt::Display for NodeBodyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assignment(assignment) => write!(f, "{assignment}"),
            Self::Expression(expression) => write!(f, "{expression}"),
            Self::Use(use_statement) => write!(f, "{use_statement}"),
        }
    }
}

/// A node body is a list of statements in curly brackets: `{a()}`
#[derive(Clone, Debug, Default)]
pub struct NodeBody {
    /// Node statements
    pub statements: Vec<NodeBodyStatement>,
    /// Node's local symbol table
    pub symbols: SymbolTable,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for NodeBody {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for NodeBody {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::node_body);

        let mut body = Self::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::module_definition_statement => {
                    let statement = NodeBodyStatement::parse(pair.clone())?;
                    body.statements.push(statement);
                }
                Rule::expression => {
                    let expression = Expression::parse(pair.clone())?;
                    body.statements
                        .push(NodeBodyStatement::Expression(expression));
                }
                _ => {}
            }
        }

        body.src_ref = pair.into();

        Ok(body)
    }
}

impl Eval for NodeBody {
    type Output = Node;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        for statement in &self.statements {
            statement.eval(context)?;
        }

        Ok(context.current_node())
    }
}

impl std::fmt::Display for NodeBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for statement in &self.statements {
            writeln!(f, "\t{statement:?}")?;
        }
        write!(f, "}}")
    }
}

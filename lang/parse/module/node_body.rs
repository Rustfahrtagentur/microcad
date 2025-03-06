// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Node body parser entity

use crate::{eval::*, parse::*, parser::*, src_ref::*, sym::*};

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
                Rule::node_body_statement => {
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
    type Output = crate::objects::ObjectNode;

    fn eval(&self, context: &mut EvalContext) -> EvalResult<Self::Output> {
        let mut group = crate::objects::group();

        for statement in &self.statements {
            match statement {
                NodeBodyStatement::Assignment(assignment) => {
                    group.add(assignment.eval(context)?);
                }
                statement => {
                    if let Some(Value::Node(node)) = statement.eval(context)? {
                        group.append(node);
                    }
                }
            }
        }

        Ok(group)
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

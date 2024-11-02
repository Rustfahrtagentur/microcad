// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Node body parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Node marker, e.g. `@children`
#[derive(Clone, Debug)]
pub struct NodeMarker {
    /// Marker name, e.g. `children`
    pub name: Identifier,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl NodeMarker {
    /// Returns true if the marker is a children marker
    pub fn is_children_marker(&self) -> bool {
        &self.name == "children"
    }
}

impl SrcReferrer for NodeMarker {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for NodeMarker {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::node_marker);
        Ok(Self {
            name: Identifier::parse(pair.inner().next().unwrap())?,
            src_ref: pair.src_ref(),
        })
    }
}

impl Eval for NodeMarker {
    type Output = Option<crate::ObjectNode>;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        match self.name.to_string().as_str() {
            "children" => Ok(Some(crate::objecttree::ObjectNode::new(
                crate::objecttree::ObjectNodeInner::ChildrenNodeMarker,
            ))),
            name => {
                use crate::diag::PushDiag;
                context.error(self, anyhow::anyhow!("Invalid node marker: {name}"))?;
                Ok(None)
            }
        }
    }
}

impl std::fmt::Display for NodeMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.name)
    }
}

/// Module initialization statement
#[derive(Clone, Debug)]
pub enum NodeBodyStatement {
    /// Node marker, e.g. @children
    NodeMarker(NodeMarker),
    /// Use statement, e.g. `use std::math::sin`
    Use(UseStatement),
    /// Expression, e.g. `a + b`
    Expression(Expression),
    /// Assignment, e.g. `a = 1`
    Assignment(Assignment),
}

impl SrcReferrer for NodeBodyStatement {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Self::NodeMarker(marker) => marker.src_ref(),
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
            Rule::node_marker => NodeBodyStatement::NodeMarker(NodeMarker::parse(first)?),
            rule => unreachable!("{rule:?}"),
        })
    }
}

impl Eval for NodeBodyStatement {
    type Output = Option<Value>;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        match self {
            Self::NodeMarker(marker) => Ok(marker.eval(context)?.map(Value::Node)),
            Self::Use(use_statement) => {
                use_statement.eval(context)?;
                Ok(None)
            }
            Self::Expression(expression) => Ok(Some(expression.eval(context)?)),
            Self::Assignment(assignment) => {
                assignment.eval(context)?;
                Ok(None)
            }
        }
    }
}

impl std::fmt::Display for NodeBodyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NodeMarker(marker) => write!(f, "{marker}"),
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
    type Output = crate::objecttree::ObjectNode;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let mut group = crate::objecttree::group();

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

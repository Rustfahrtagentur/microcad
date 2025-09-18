// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for Assignment {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self {
            visibility: crate::find_rule!(pair, visibility)?,
            qualifier: crate::find_rule!(pair, qualifier)?,
            id: crate::find_rule!(pair, identifier)?,
            specified_type: crate::find_rule_opt!(pair, r#type),
            expression: crate::find_rule!(pair, expression)?,
            src_ref: pair.into(),
        })
    }
}

impl Parse for AssignmentStatement {
    fn parse(pair: Pair) -> crate::parse::ParseResult<Self> {
        Ok(Self {
            attribute_list: crate::find_rule!(pair, attribute_list)?,
            assignment: crate::find_rule_opt!(pair, assignment).expect("Assignment"),
            src_ref: pair.into(),
        })
    }
}

impl Parse for IfStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut cond = Default::default();
        let mut body = None;
        let mut body_else = None;
        let mut next_if = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::expression => cond = Expression::parse(pair)?,
                Rule::body => {
                    if body.is_none() {
                        body = Some(Body::parse(pair)?)
                    } else {
                        body_else = Some(Body::parse(pair)?)
                    }
                }
                Rule::if_statement => {
                    if next_if.is_none() {
                        next_if = Some(Box::new(IfStatement::parse(pair)?));
                    }
                }
                rule => unreachable!("Unexpected rule in if, got {:?}", rule),
            }
        }

        let body = body.expect("Body");

        Ok(IfStatement {
            cond,
            body,
            body_else,
            next_if,
            src_ref: pair.into(),
        })
    }
}

impl Parse for ExpressionStatement {
    fn parse(pair: Pair) -> crate::parse::ParseResult<Self> {
        Parser::ensure_rules(
            &pair,
            &[Rule::expression_statement, Rule::final_expression_statement],
        );

        Ok(Self {
            attribute_list: crate::find_rule!(pair, attribute_list)?,
            expression: pair.find(Rule::expression).expect("Expression"),
            src_ref: pair.into(),
        })
    }
}

impl Parse for Statement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::statement);
        let first = pair.inner().next().expect(INTERNAL_PARSE_ERROR);
        Ok(match first.as_rule() {
            Rule::workbench_definition => Self::Workbench(Rc::<WorkbenchDefinition>::parse(first)?),
            Rule::module_definition => Self::Module(Rc::<ModuleDefinition>::parse(first)?),
            Rule::function_definition => Self::Function(Rc::<FunctionDefinition>::parse(first)?),
            Rule::init_definition => Self::Init(Rc::new(InitDefinition::parse(first)?)),

            Rule::use_statement => Self::Use(UseStatement::parse(first)?),
            Rule::return_statement => Self::Return(ReturnStatement::parse(first)?),
            Rule::if_statement => Self::If(IfStatement::parse(first)?),
            Rule::inner_attribute => Self::InnerAttribute(Attribute::parse(first)?),

            Rule::assignment_statement => Self::Assignment(AssignmentStatement::parse(first)?),
            Rule::expression_statement | Rule::final_expression_statement => {
                Self::Expression(ExpressionStatement::parse(first)?)
            }
            rule => unreachable!("Unexpected statement, got {:?} {:?}", rule, first.clone()),
        })
    }
}

impl Parse for ReturnStatement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut result = None;

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::expression => result = Some(Expression::parse(pair)?),
                rule => unreachable!("Unexpected rule in return, got {:?}", rule),
            }
        }

        Ok(ReturnStatement {
            result,
            src_ref: pair.into(),
        })
    }
}

impl Parse for StatementList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut statements = Vec::new();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::final_expression_statement => {
                    statements.push(Statement::Expression(ExpressionStatement::parse(pair)?));
                }
                Rule::statement => {
                    statements.push(Statement::parse(pair)?);
                }
                _ => {}
            }
        }

        Ok(Self(statements))
    }
}

impl Parse for Qualifier {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::qualifier);
        match pair.as_str() {
            "prop" => Ok(Self::Prop),
            "const" => Ok(Self::Const),
            _ => Ok(Self::Value),
        }
    }
}

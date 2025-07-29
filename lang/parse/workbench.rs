// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for WorkbenchKind {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.as_str() {
            "part" => Ok(Self::Part),
            "sketch" => Ok(Self::Sketch),
            "op" => Ok(Self::Operation),
            _ => Err(ParseError::UnexpectedToken),
        }
    }
}

impl Parse for Rc<WorkbenchDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(WorkbenchDefinition {
            visibility: crate::find_rule!(pair, visibility)?,
            attribute_list: crate::find_rule!(pair, attribute_list)?,
            kind: crate::find_rule_exact!(pair, workbench_kind)?,
            id: crate::find_rule!(pair, identifier)?,
            plan: crate::find_rule!(pair, parameter_list)?,
            body: {
                let body = crate::find_rule!(pair, body)?;
                check_statements(&body)?;
                body
            },
            src_ref: pair.into(),
        }
        .into())
    }
}

fn check_statements(body: &Body) -> ParseResult<()> {
    if let (Some(first_init), Some(last_init)) = (
        body.iter()
            .position(|stmt| matches!(stmt, Statement::Init(_))),
        body.iter()
            .rposition(|stmt| matches!(stmt, Statement::Init(_))),
    ) {
        for (n, stmt) in body.iter().enumerate() {
            match stmt {
                // ignore inits
                Statement::Init(_) => (),

                // RULE: Illegal statements in workbenches
                Statement::Module(_) | Statement::Workbench(_) | Statement::Return(_) => {
                    return Err(ParseError::IllegalWorkbenchStatement);
                }

                // RULE: Ony use or assignments before initializers
                Statement::Use(_) => {
                    if n > first_init && n < last_init {
                        return Err(ParseError::CodeBetweenInitializers);
                    }
                }

                // Some assignments are post init statements
                Statement::Assignment(assignment) => match assignment.assignment.qualifier {
                    Qualifier::Const => {
                        if n > first_init && n < last_init {
                            return Err(ParseError::CodeBetweenInitializers);
                        }
                    }
                    Qualifier::Var | Qualifier::Prop => {
                        if n < last_init {
                            if n > first_init {
                                return Err(ParseError::CodeBetweenInitializers);
                            }
                            return Err(ParseError::StatementNotAllowedPriorInitializers);
                        }
                    }
                },
                Statement::ModelAssignment(_) => todo!(),

                // Post init statements
                Statement::If(_)
                | Statement::InnerAttribute(_)
                | Statement::Expression(_)
                | Statement::Function(_) => {
                    // RULE: No code between initializers
                    if n < last_init {
                        if n > first_init {
                            return Err(ParseError::CodeBetweenInitializers);
                        }
                        return Err(ParseError::StatementNotAllowedPriorInitializers);
                    }
                }
            }
        }
    }
    Ok(())
}

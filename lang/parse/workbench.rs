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
        Ok(WorkbenchDefinition::new(
            pair.find(Rule::attribute_list).unwrap_or_default(),
            pair.find(Rule::workbench_kind).expect("workbench kind"),
            pair.find(Rule::identifier).expect("workbench identifier"),
            pair.find(Rule::parameter_list)
                .expect("workbench parameter_list"),
            {
                let body = pair.find(Rule::body).expect("workbench body");
                check_statements(&body)?;
                body
            },
            pair.into(),
        ))
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
                Statement::Function(_) | Statement::Use(_) | Statement::Assignment(_) => {
                    if n > first_init {
                        return Err(ParseError::CodeBetweenInitializers);
                    }
                }

                // Post init statements
                Statement::If(_) | Statement::Marker(_) | Statement::Expression(_) => {
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

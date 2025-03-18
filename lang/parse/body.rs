// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module body parser entity

use crate::{objects::*, parse::*, parser::*, src_ref::*};

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
    /// Evaluate a single statement of the module
    fn eval_statement(
        &self,
        statement: &Statement,
        context: &mut EvalContext,
        group: &mut ObjectNode,
    ) -> EvalResult<()> {
        match statement {
            Statement::Assignment(assignment) => {
                // Evaluate the assignment and add the symbol to the node
                // E.g. `a = 1` will add the symbol `a` to the node
                let symbol = assignment.eval(context)?;
                group.add(symbol);
            }
            Statement::FunctionDefinition(function) => {
                // Evaluate the function and add the symbol to the node
                // E.g. `function a() {}` will add the symbol `a` to the node
                let symbol = function.eval(context)?;
                group.add(symbol);
            }
            statement => {
                if let Some(Value::Node(new_child)) = statement.eval(context)? {
                    group.append(new_child);
                }
            }
        }
        Ok(())
    }
}

impl SrcReferrer for Body {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for Body {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_body);
        let mut body = Self::default();
        for pair in pair.inner() {
            body.statements.push(Statement::parse(pair.clone())?);
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

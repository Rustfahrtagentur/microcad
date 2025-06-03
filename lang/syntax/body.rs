// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Body syntax element.

use crate::{eval::*, objects::*, resolve::*, src_ref::*, syntax::*, value::*};

/// A body is a list of statements inside `{}` brackets.
#[derive(Clone, Debug, Default)]
pub struct Body {
    /// Body statements.
    pub statements: StatementList,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl Body {
    /// fetches all symbols from the statements in the body.
    pub fn resolve(&self, parent: Option<Symbol>) -> SymbolMap {
        self.statements.fetch_symbol_map(parent)
    }

    /// Evaluate a vector of statements.
    pub fn evaluate_vec(statements: &Vec<Statement>, context: &mut Context) -> EvalResult<Value> {
        for s in statements {
            s.eval(context)?;
        }
        Ok(Value::None)
    }

    /// Evaluate the statement of this body into an ObjectNode.
    pub fn eval_to_node(&self, context: &mut Context) -> EvalResult<ObjectNode> {
        context.scope(StackFrame::Body(SymbolMap::default()), |context| {
            let mut nodes = Vec::new();

            for statement in self.statements.iter() {
                let value = match statement {
                    Statement::Use(_) => continue, // Use statements have been resolved at this point
                    Statement::Assignment(assignment) => assignment.eval(context)?,
                    Statement::Expression(expression) => expression.eval(context)?,
                    Statement::Marker(marker) => marker.eval(context)?,
                    Statement::If(_) => todo!("if statement not implemented"),
                    statement => {
                        use crate::diag::PushDiag;
                        context.error(
                            self,
                            EvalError::StatementNotSupported(statement.clone().into()),
                        )?;
                        Value::None
                    }
                };

                nodes.append(&mut value.fetch_nodes());
            }

            let object = empty_object();
            for node in nodes {
                object.append(node);
            }

            Ok(object)
        })
    }
}

impl SrcReferrer for Body {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, " {{")?;
        writeln!(f, "{}", self.statements)?;
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

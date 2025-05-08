// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module body syntax element.

use crate::{eval::*, objects::*, resolve::*, src_ref::*, syntax::*, value::*};

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
    /// Module statements.
    pub statements: Vec<Statement>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl Body {
    /// fetches all symbols from a slice of statements.
    pub fn fetch_symbol_map(statements: &[Statement], parent: Option<Symbol>) -> SymbolMap {
        let mut symbol_map = SymbolMap::default();

        // Iterate over all statement fetch definitions
        for statement in statements {
            match statement {
                Statement::Module(m) => {
                    symbol_map.insert(m.id.clone(), m.resolve(parent.clone()));
                }
                Statement::Namespace(n) => {
                    symbol_map.insert(n.id.clone(), n.resolve(parent.clone()));
                }
                Statement::Function(f) => {
                    symbol_map.insert(f.id.clone(), f.resolve(parent.clone()));
                }
                Statement::Use(u) => symbol_map.append(&mut u.resolve(parent.clone())),
                _ => {}
            }
        }

        symbol_map
    }

    /// fetches all symbols from the statements in the body.
    pub fn resolve(&self, parent: Option<Symbol>) -> SymbolMap {
        Self::fetch_symbol_map(&self.statements, parent)
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

            for statement in &self.statements {
                let value = match statement {
                    Statement::Use(_) => continue, // Use statements have been resolved at this point
                    Statement::Assignment(assignment) => assignment.eval(context)?,
                    Statement::Expression(expression) => expression.eval(context)?,
                    Statement::Marker(marker) => marker.eval(context)?,
                    Statement::If(_) => todo!("if statement not implemented"),
                    statement => {
                        use crate::diag::PushDiag;
                        context.error(self, EvalError::StatementNotSupported(statement.clone()))?;
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

        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }

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

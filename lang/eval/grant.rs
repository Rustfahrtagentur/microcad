// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, syntax::*};

/// Grant statements depending on context
pub trait Grant<T> {
    /// Check if given statement `T` is granted within the current context
    fn grant(&mut self, t: &T) -> EvalResult<()>;
}

impl Grant<WorkbenchDefinition> for EvalContext {
    fn grant(&mut self, statement: &WorkbenchDefinition) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(_, _) | StackFrame::Module(_, _)
            )
        } else {
            false
        };
        if !granted {
            self.error(
                statement,
                EvalError::StatementNotSupported(statement.kind.as_str()),
            )?;
        }
        Ok(())
    }
}

impl Grant<ModuleDefinition> for EvalContext {
    fn grant(&mut self, statement: &ModuleDefinition) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(_, _) | StackFrame::Module(_, _)
            )
        } else {
            false
        };
        if !granted {
            self.error(statement, EvalError::StatementNotSupported("Module"))?;
        }
        Ok(())
    }
}

impl Grant<FunctionDefinition> for EvalContext {
    fn grant(&mut self, statement: &FunctionDefinition) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
            match stack_frame {
                // TODO: check if expression generates models (see test `source_expression``)
                StackFrame::Source(..) | StackFrame::Module(..) => true,
                StackFrame::Workbench(..) => statement.visibility == Visibility::Private,
                _ => false,
            }
        } else {
            false
        };
        if !granted {
            self.error(statement, EvalError::StatementNotSupported("Function"))?;
        }
        Ok(())
    }
}
impl Grant<InitDefinition> for EvalContext {
    fn grant(&mut self, statement: &InitDefinition) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
            matches!(stack_frame, StackFrame::Workbench(..))
        } else {
            false
        };
        if !granted {
            self.error(statement, EvalError::StatementNotSupported("Init"))?;
        }
        Ok(())
    }
}

impl Grant<UseStatement> for EvalContext {
    fn grant(&mut self, statement: &UseStatement) -> EvalResult<()> {
        match (&statement.visibility, self.stack.current_frame()) {
            (Visibility::Private, _)
            | (Visibility::Public, Some(StackFrame::Source(..) | StackFrame::Module(..))) => (),
            _ => self.error(statement, EvalError::StatementNotSupported("Public use"))?,
        }
        Ok(())
    }
}

impl Grant<ReturnStatement> for EvalContext {
    fn grant(&mut self, statement: &ReturnStatement) -> EvalResult<()> {
        if !self.is_within_function() {
            self.error(statement, EvalError::StatementNotSupported("Return"))?;
        }
        Ok(())
    }
}

impl Grant<IfStatement> for EvalContext {
    fn grant(&mut self, statement: &IfStatement) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(_, _)
                    | StackFrame::Workbench(_, _, _)
                    | StackFrame::Body(_)
                    | StackFrame::Function(_)
            )
        } else {
            false
        };
        if !granted {
            self.error(statement, EvalError::StatementNotSupported("If"))?;
        }
        Ok(())
    }
}

impl Grant<AssignmentStatement> for EvalContext {
    fn grant(&mut self, statement: &AssignmentStatement) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
            match statement.assignment.qualifier() {
                Qualifier::Const => {
                    matches!(stack_frame, StackFrame::Source(..) | StackFrame::Module(..))
                }
                Qualifier::Value => {
                    matches!(
                        stack_frame,
                        StackFrame::Source(..)
                            | StackFrame::Module(..)
                            | StackFrame::Body(_)
                            | StackFrame::Workbench(..)
                            | StackFrame::Init(_)
                            | StackFrame::Function(_)
                    )
                }
                Qualifier::Prop => matches!(stack_frame, StackFrame::Workbench(..)),
            }
        } else {
            false
        };
        if !granted {
            self.error(statement, EvalError::StatementNotSupported("Assignment"))?;
        }
        Ok(())
    }
}

impl Grant<ExpressionStatement> for EvalContext {
    fn grant(&mut self, statement: &ExpressionStatement) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(_, _)
                    | StackFrame::Body(_)
                    | StackFrame::Workbench(_, _, _)
                    | StackFrame::Function(_)
            )
        } else {
            false
        };
        if !granted {
            self.error(statement, EvalError::StatementNotSupported("Expression"))?;
        }
        Ok(())
    }
}

impl Grant<Marker> for EvalContext {
    fn grant(&mut self, statement: &Marker) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
            matches!(stack_frame, StackFrame::Workbench(_, _, _))
        } else {
            false
        };
        if !granted {
            self.error(statement, EvalError::StatementNotSupported("Expression"))?;
        }
        Ok(())
    }
}

impl Grant<Attribute> for EvalContext {
    fn grant(&mut self, statement: &crate::syntax::Attribute) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(_, _) | StackFrame::Body(_) | StackFrame::Workbench(_, _, _)
            )
        } else {
            false
        };
        if !granted {
            self.error(
                statement,
                EvalError::StatementNotSupported("InnerAttribute"),
            )?;
        }
        Ok(())
    }
}

impl Grant<Body> for EvalContext {
    fn grant(&mut self, body: &Body) -> EvalResult<()> {
        let granted = if let Some(stack_frame) = self.stack.current_frame() {
            matches!(
                stack_frame,
                StackFrame::Source(_, _) | StackFrame::Body(_) | StackFrame::Workbench(_, _, _)
            )
        } else {
            false
        };
        if !granted {
            self.error(body, EvalError::StatementNotSupported("Body"))?;
        }
        Ok(())
    }
}

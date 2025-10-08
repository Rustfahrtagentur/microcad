// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Check wether a statement is legally placed.

use crate::{resolve::*, syntax::*};

fn capitalize_first(s: String) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub(super) trait Grant {
    /// Checks if definition is allowed to occur within the given parent symbol.
    fn grant(&self, _parent: &Symbol, _context: &mut ResolveContext) -> DiagResult<&Self> {
        Ok(self)
    }
}

impl Grant for ModuleDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..) | SymbolDefinition::Module(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported(
                    "Module definition".into(),
                    capitalize_first(def.kind()),
                ),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for StatementList {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..)
            | SymbolDefinition::Module(..)
            | SymbolDefinition::Workbench(..)
            | SymbolDefinition::Function(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported("Statement list".to_string(), parent.kind()),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for Statement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match (def, &self) {
            (
                SymbolDefinition::SourceFile(..),
                Statement::Assignment(..)
                | Statement::Expression(..)
                | Statement::Function(..)
                | Statement::If(..)
                | Statement::Module(..)
                | Statement::Use(..)
                | Statement::Workbench(..),
            )
            | (
                SymbolDefinition::Module(..),
                Statement::Assignment(..)
                | Statement::Expression(..)
                | Statement::Function(..)
                | Statement::Module(..)
                | Statement::Use(..)
                | Statement::Workbench(..),
            )
            | (
                SymbolDefinition::Workbench(..),
                Statement::Assignment(..)
                | Statement::Expression(..)
                | Statement::Function(..)
                | Statement::If(..)
                | Statement::Init(..)
                | Statement::Use(..),
            )
            | (
                SymbolDefinition::Function(..),
                Statement::Assignment(..)
                | Statement::If(..)
                | Statement::Return(..)
                | Statement::Use(..),
            ) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported("Statement".to_string(), parent.kind()),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for WorkbenchDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..) | SymbolDefinition::Module(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported(self.kind.to_string(), parent.kind()),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for FunctionDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..)
            | SymbolDefinition::Module(..)
            | SymbolDefinition::Workbench(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported(
                    "Function definition".to_string(),
                    parent.kind(),
                ),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for InitDefinition {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::Workbench(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported("Init definition".to_string(), parent.kind()),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for ReturnStatement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::Function(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported("Return statement".to_string(), parent.kind()),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for IfStatement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..)
            | SymbolDefinition::Module(..)
            | SymbolDefinition::Workbench(..)
            | SymbolDefinition::Function(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported("If statement".to_string(), parent.kind()),
            ),
        })?;
        Ok(self)
    }
}

impl Grant for AssignmentStatement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        let grant = parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..) | SymbolDefinition::Module(..) => {
                match self.assignment.qualifier {
                    Qualifier::Value => matches!(self.assignment.visibility, Visibility::Private),
                    Qualifier::Const => true,
                    Qualifier::Prop => false,
                }
            }
            SymbolDefinition::Workbench(..) | SymbolDefinition::Function(..) => {
                match self.assignment.qualifier {
                    Qualifier::Value | Qualifier::Prop => {
                        matches!(self.assignment.visibility, Visibility::Private)
                    }
                    Qualifier::Const => false,
                }
            }
            _ => false,
        });

        if !grant {
            context.error(
                self,
                ResolveError::StatementNotSupported(
                    "Assignment statement".to_string(),
                    parent.kind(),
                ),
            )?;
        }
        Ok(self)
    }
}

impl Grant for Body {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..)
            | SymbolDefinition::Module(..)
            | SymbolDefinition::Workbench(..)
            | SymbolDefinition::Function(..) => Ok(()),
            _ => context.error(
                self,
                ResolveError::StatementNotSupported("Code body".to_string(), parent.kind()),
            ),
        })?;
        Ok(self)
    }
}
impl Grant for UseStatement {
    fn grant(&self, parent: &Symbol, context: &mut ResolveContext) -> DiagResult<&Self> {
        let grant = parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..) | SymbolDefinition::Module(..) => true,
            SymbolDefinition::Workbench(..) | SymbolDefinition::Function(..) => {
                match self.visibility {
                    Visibility::Private => true,
                    Visibility::Public => false,
                }
            }
            _ => false,
        });

        if !grant {
            context.error(
                self,
                ResolveError::StatementNotSupported("Use statement".to_string(), parent.kind()),
            )?;
        }
        Ok(self)
    }
}

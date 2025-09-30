// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Check wether a statement is legally placed.

use crate::{resolve::*, syntax::*};

pub(super) trait Grant {
    /// Checks if definition is allowed to occur within the given parent symbol.
    fn grant(&self, _parent: &Symbol) -> ResolveResult<&Self> {
        Ok(self)
    }
}

impl Grant for ModuleDefinition {
    fn grant(&self, parent: &Symbol) -> ResolveResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..) | SymbolDefinition::Module(..) => Ok(self),
            _ => Err(ResolveError::StatementNotSupported(
                self.to_string(),
                parent.to_string(),
            )),
        })
    }
}

impl Grant for StatementList {
    fn grant(&self, parent: &Symbol) -> ResolveResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..)
            | SymbolDefinition::Module(..)
            | SymbolDefinition::Workbench(..)
            | SymbolDefinition::Function(..) => Ok(self),
            _ => Err(ResolveError::StatementNotSupported(
                self.to_string(),
                parent.to_string(),
            )),
        })
    }
}

impl Grant for Statement {
    fn grant(&self, parent: &Symbol) -> ResolveResult<&Self> {
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
            ) => Ok(self),
            _ => Err(ResolveError::StatementNotSupported(
                self.to_string(),
                parent.to_string(),
            )),
        })
    }
}

impl Grant for WorkbenchDefinition {
    fn grant(&self, parent: &Symbol) -> ResolveResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..) | SymbolDefinition::Module(..) => Ok(self),
            _ => Err(ResolveError::StatementNotSupported(
                self.to_string(),
                parent.to_string(),
            )),
        })
    }
}

impl Grant for FunctionDefinition {
    fn grant(&self, parent: &Symbol) -> ResolveResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..)
            | SymbolDefinition::Module(..)
            | SymbolDefinition::Workbench(..) => Ok(self),
            _ => Err(ResolveError::StatementNotSupported(
                self.to_string(),
                parent.to_string(),
            )),
        })
    }
}

impl Grant for InitDefinition {
    fn grant(&self, parent: &Symbol) -> ResolveResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::Workbench(..) => Ok(self),
            _ => Err(ResolveError::StatementNotSupported(
                self.to_string(),
                parent.to_string(),
            )),
        })
    }
}

impl Grant for ReturnStatement {
    fn grant(&self, parent: &Symbol) -> ResolveResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::Function(..) => Ok(self),
            _ => Err(ResolveError::StatementNotSupported(
                self.to_string(),
                parent.to_string(),
            )),
        })
    }
}

impl Grant for IfStatement {
    fn grant(&self, parent: &Symbol) -> ResolveResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..)
            | SymbolDefinition::Module(..)
            | SymbolDefinition::Workbench(..)
            | SymbolDefinition::Function(..) => Ok(self),
            _ => Err(ResolveError::StatementNotSupported(
                self.to_string(),
                parent.to_string(),
            )),
        })
    }
}

impl Grant for AssignmentStatement {
    fn grant(&self, parent: &Symbol) -> ResolveResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..) | SymbolDefinition::Module(..) => Ok(self),
            SymbolDefinition::Workbench(..) | SymbolDefinition::Function(..) => {
                match self.assignment.visibility {
                    Visibility::Private => Ok(self),
                    Visibility::Public => Err(ResolveError::StatementNotSupported(
                        self.to_string(),
                        parent.to_string(),
                    )),
                }
            }
            _ => Err(ResolveError::StatementNotSupported(
                self.to_string(),
                parent.to_string(),
            )),
        })
    }
}

impl Grant for Body {
    fn grant(&self, parent: &Symbol) -> ResolveResult<&Self> {
        parent.with_def(|def| match def {
            SymbolDefinition::SourceFile(..)
            | SymbolDefinition::Module(..)
            | SymbolDefinition::Workbench(..)
            | SymbolDefinition::Function(..) => Ok(self),
            _ => Err(ResolveError::StatementNotSupported(
                self.to_string(),
                parent.to_string(),
            )),
        })
    }
}
impl Grant for UseStatement {
    fn grant(&self, parent: &Symbol) -> ResolveResult<&Self> {
        parent.with_def(|def| -> Result<&UseStatement, ResolveError> {
            match def {
                SymbolDefinition::SourceFile(..) | SymbolDefinition::Module(..) => Ok(self),
                SymbolDefinition::Workbench(..) | SymbolDefinition::Function(..) => {
                    match self.visibility {
                        Visibility::Private => Ok(self),
                        Visibility::Public => Err(ResolveError::StatementNotSupported(
                            self.to_string(),
                            parent.to_string(),
                        )),
                    }
                }
                _ => Err(ResolveError::StatementNotSupported(
                    self.to_string(),
                    parent.to_string(),
                )),
            }
        })
    }
}

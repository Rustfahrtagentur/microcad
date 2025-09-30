// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Single symbol resolving
//!
//! After parsing a source file (see [`mod@crate::parse`]) it must be resolved to get a symbol out of it:
//!
//! ```no_run
//! use microcad_lang::{syntax::*, parse::*, resolve::*};
//!
//! let source_file = SourceFile::load("my.µcad").expect("parsing success");
//!
//! let source_symbol = source_file.resolve();
//! ```
//!
//! To "run" the source file (and get the expected output) it must now be evaluated (see [`crate::eval`])  .

mod externals;
mod resolve_context;
mod resolve_error;
mod sources;
mod symbol;
mod symbol_definition;
mod symbol_map;
mod symbol_table;

pub use externals::*;
pub use resolve_context::*;
pub use resolve_error::*;
pub use sources::*;
pub use symbol::*;
pub use symbol_definition::*;
pub use symbol_map::*;
pub use symbol_table::*;

use crate::{diag::*, syntax::*, value::Value};

/// Trait to handle symbol table.
pub trait Lookup<E: std::error::Error = ResolveError> {
    /// Lookup for local or global symbol by qualified name.
    ///
    /// - looks on *stack*
    /// - looks in *symbol table*
    /// - follows *aliases* (use statements)
    /// - detect any ambiguity
    ///
    /// # Arguments
    /// -`name`: qualified name to search for
    fn lookup(&self, name: &QualifiedName) -> Result<Symbol, E>;
}

/// Trait for items which can be fully qualified.
pub trait FullyQualify {
    /// Get a fully (up to root of symbol map) qualified name.
    fn full_name(&self) -> QualifiedName;
}

trait Resolve<T: Default = Option<Symbol>> {
    /// Checks if definition is allowed to occur within the given parent symbol.
    fn grant(&self, _parent: &Symbol) -> ResolveResult<&Self> {
        Ok(self)
    }

    /// Resolve into Symbol
    fn symbolize(&self, _parent: &Symbol, _context: &mut ResolveContext) -> ResolveResult<T> {
        Ok(T::default())
    }

    fn names(&self) -> Vec<&QualifiedName> {
        Vec::new()
    }
}

#[test]
fn resolve_test() {
    let root =
        SourceFile::load("../examples/my_brick.µcad").expect("loading of root source file failed");
    log::trace!("Root source file:\n{root}");

    let mut symbol_table = SymbolTable::load(root, &["../lib"], DiagHandler::default())
        .expect("loading of symbol table failed");

    symbol_table.resolve().expect("resolve failed");

    symbol_table.check().expect("check failed");
}

impl SourceFile {
    /// Resolve into Symbol
    pub fn symbolize(
        &self,
        visibility: Visibility,
        context: &mut ResolveContext,
    ) -> ResolveResult<Symbol> {
        let symbol = Symbol::new_with_visibility(
            visibility,
            SymbolDefinition::SourceFile(self.clone().into()),
            None,
        );
        symbol.set_children(
            self.statements
                .grant(&symbol)?
                .symbolize(&symbol, context)?,
        );
        Ok(symbol)
    }

    pub fn names(&self) -> Vec<&QualifiedName> {
        self.statements.names()
    }
}

impl Resolve<Symbol> for ModuleDefinition {
    /// Resolve into Symbol
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<Symbol> {
        let symbol = if let Some(body) = &self.body {
            let symbol = Symbol::new(
                SymbolDefinition::Module(self.clone().into()),
                Some(parent.clone()),
            );
            symbol.set_children(body.grant(&symbol)?.symbolize(&symbol, context)?);
            symbol
        } else if let Some(parent_path) = parent.source_path() {
            let mut symbol = context.symbolize_file(self.visibility, parent_path, &self.id)?;
            symbol.set_parent(parent.clone());
            symbol
        } else {
            todo!("no top-level source file")
        };
        Ok(symbol)
    }

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

impl Resolve<SymbolMap> for StatementList {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<SymbolMap> {
        let mut symbols = SymbolMap::default();

        // Iterate over all statement fetch definitions
        for statement in &self.0 {
            if let Some((id, symbol)) = statement.grant(parent)?.symbolize(parent, context)? {
                symbols.insert(id, symbol);
            }
        }

        Ok(symbols)
    }

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

impl Resolve<Option<(Identifier, Symbol)>> for Statement {
    fn symbolize(
        &self,
        parent: &Symbol,
        context: &mut ResolveContext,
    ) -> ResolveResult<Option<(Identifier, Symbol)>> {
        match self {
            Statement::Workbench(wd) => Ok(Some((
                wd.id.clone(),
                wd.grant(parent)?.symbolize(parent, context)?,
            ))),
            Statement::Module(md) => Ok(Some((
                md.id.clone(),
                md.grant(parent)?.symbolize(parent, context)?,
            ))),
            Statement::Function(fd) => Ok(Some((
                fd.id.clone(),
                fd.grant(parent)?.symbolize(parent, context)?,
            ))),
            Statement::Use(us) => us.grant(parent)?.symbolize(parent, context),
            Statement::Assignment(a) => Ok(a
                .grant(parent)?
                .symbolize(parent, context)?
                .map(|symbol| (a.assignment.id.clone(), symbol))),
            // Not producing any symbols
            Statement::Init(_)
            | Statement::Return(_)
            | Statement::If(_)
            | Statement::InnerAttribute(_)
            | Statement::Expression(_) => Ok(None),
        }
    }
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

impl Resolve<Symbol> for WorkbenchDefinition {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(
            SymbolDefinition::Workbench(self.clone().into()),
            Some(parent.clone()),
        );
        symbol.set_children(self.body.grant(&symbol)?.symbolize(&symbol, context)?);
        Ok(symbol)
    }

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

impl Resolve<Symbol> for FunctionDefinition {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(
            SymbolDefinition::Function((*self).clone().into()),
            Some(parent.clone()),
        );
        symbol.set_children(self.body.grant(&symbol)?.symbolize(&symbol, context)?);

        Ok(symbol)
    }

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

impl Resolve for InitDefinition {
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

impl Resolve for ReturnStatement {
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

impl Resolve for IfStatement {
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

impl Resolve for Attribute {}

impl Resolve for AssignmentStatement {
    fn symbolize(
        &self,
        parent: &Symbol,
        context: &mut ResolveContext,
    ) -> ResolveResult<Option<Symbol>> {
        match (self.assignment.visibility, self.assignment.qualifier) {
            // properties do not have a visibility
            (_, Qualifier::Prop) => {
                if !parent.can_prop() {
                    context.error(
                        self,
                        ResolveError::DeclNotAllowed(
                            self.assignment.id.clone(),
                            parent.full_name(),
                        ),
                    )?;
                }
                Ok(None)
            }
            // constants will be symbols (`pub` shall equal `pub const`)
            (_, Qualifier::Const) | (Visibility::Public, Qualifier::Value) => {
                if !parent.can_const() {
                    Err(ResolveError::DeclNotAllowed(
                        self.assignment.id.clone(),
                        parent.full_name(),
                    ))
                } else {
                    log::trace!("Declare private value {}", self.assignment.id);
                    Ok(Some(Symbol::new(
                        SymbolDefinition::Constant(
                            self.assignment.visibility,
                            self.assignment.id.clone(),
                            Value::None,
                        ),
                        Some(parent.clone()),
                    )))
                }
            }
            // value go on stack
            (Visibility::Private, Qualifier::Value) => {
                if self.assignment.visibility == Visibility::Private && !parent.can_value() {
                    Err(ResolveError::DeclNotAllowed(
                        self.assignment.id.clone(),
                        parent.full_name(),
                    ))
                } else {
                    Ok(None)
                }
            }
        }
    }

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

impl Resolve for ExpressionStatement {}

impl Resolve<SymbolMap> for Body {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<SymbolMap> {
        self.statements.grant(parent)?.symbolize(parent, context)
    }
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

impl Resolve<Option<(Identifier, Symbol)>> for UseStatement {
    fn symbolize(
        &self,
        parent: &Symbol,
        _: &mut ResolveContext,
    ) -> ResolveResult<Option<(Identifier, Symbol)>> {
        match &self.decl {
            UseDeclaration::Use(name) => {
                let identifier = name.last().expect("Identifier");
                Ok(Some((
                    identifier.clone(),
                    Symbol::new(
                        SymbolDefinition::Alias(self.visibility, identifier.clone(), name.clone()),
                        Some(parent.clone()),
                    ),
                )))
            }
            UseDeclaration::UseAll(name) => Ok(Some((
                Identifier::unique(),
                Symbol::new(
                    SymbolDefinition::UseAll(self.visibility, name.clone()),
                    Some(parent.clone()),
                ),
            ))),
            UseDeclaration::UseAlias(name, alias) => Ok(Some((
                alias.clone(),
                Symbol::new(
                    SymbolDefinition::Alias(self.visibility, alias.clone(), name.clone()),
                    Some(parent.clone()),
                ),
            ))),
        }
    }

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

#[test]
fn resolve_source_file() {
    todo!()
    /*  let source_file =
        SourceFile::load_from_str(r#"part A() { part B() {} } "#).expect("Valid source");

    let symbol = source_file.resolve().expect("expecting resolve success");

    // file <no file>
    //  part a
    //   part b
    assert!(symbol.get(&"A".into()).is_some());
    assert!(symbol.get(&"c".into()).is_none());

    assert!(symbol.search(&"A".into()).is_some());
    assert!(symbol.search(&"A::B".into()).is_some());
    assert!(symbol.search(&"A::B::C".into()).is_none());

    // use std::print; // Add symbol "print" to current symbol symbol
    // part M() {
    //      print("test"); // Use symbol symbol from parent
    // }

    log::trace!("Symbol symbol:\n{symbol}");

    let b = symbol.search(&"A::B".into()).expect("cant find symbol");
    assert!(b.search(&"A".into()).is_none());

    //assert!(symbol_node.search_top_down(&["<no file>".into()]).is_some());

    log::trace!("{symbol}");
    */
}

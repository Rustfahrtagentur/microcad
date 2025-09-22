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
mod resolve_error;
mod sources;
mod symbol;
mod symbol_definition;
mod symbol_map;
mod symbol_table;

pub use externals::*;
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
    /// - loads *external files*
    fn lookup(&self, name: &QualifiedName) -> Result<Symbol, E>;
}

/// Trait for items which can be fully qualified.
pub trait FullyQualify {
    /// Get a fully (up to root of symbol map) qualified name.
    fn full_name(&self) -> QualifiedName;
}

trait Resolve<T = Option<Symbol>> {
    /// Resolve into Symbol
    fn symbolize(&self, parent: &Symbol, diag: &DiagHandler) -> ResolveResult<T>;

    //    fn resolve2(&self, parent: &Symbol
}

#[test]
fn resolve_test() {
    let root = SourceFile::load("../examples/lego_brick.µcad")
        .expect("loading of root source file failed");
    log::trace!("Root source file:\n{root}");

    let mut symbol_table = SymbolTable::load(root, &["../lib"], DiagHandler::default())
        .expect("loading of symbol table failed");
    log::trace!("Initial symbol table:\n{symbol_table}");

    symbol_table.resolve().expect("resolve failed");
    log::trace!("Resolved symbol table:\n{symbol_table}");

    symbol_table.check().expect("check failed");
}

impl SourceFile {
    /// Resolve into Symbol
    pub fn symbolize(&self, diag: &DiagHandler) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(SymbolDefinition::SourceFile(self.clone().into()), None);
        symbol.set_children(self.statements.symbolize(&symbol, diag)?);
        Ok(symbol)
    }
}

impl Resolve<Symbol> for ModuleDefinition {
    /// Resolve into Symbol
    fn symbolize(&self, parent: &Symbol, diag: &DiagHandler) -> ResolveResult<Symbol> {
        let symbol = if let Some(body) = &self.body {
            let symbol = Symbol::new(
                SymbolDefinition::Module(self.clone().into()),
                Some(parent.clone()),
            );
            symbol.set_children(body.symbolize(&symbol, diag)?);
            symbol
        } else if let Some(parent_path) = parent.source_path() {
            let file_path = find_source_file(parent_path, &self.id)?;
            let source_file = SourceFile::load(file_path)?;
            source_file.symbolize(diag)?
        } else {
            todo!("no top-level source file")
        };
        Ok(symbol)
    }
}

impl Resolve<SymbolMap> for StatementList {
    fn symbolize(&self, parent: &Symbol, diag: &DiagHandler) -> ResolveResult<SymbolMap> {
        let mut symbols = SymbolMap::default();

        // Iterate over all statement fetch definitions
        for statement in &self.0 {
            if let Some((id, symbol)) = statement.symbolize(parent, diag)? {
                symbols.insert(id, symbol);
            }
        }

        Ok(symbols)
    }
}

impl Resolve<Option<(Identifier, Symbol)>> for Statement {
    fn symbolize(
        &self,
        parent: &Symbol,
        diag: &DiagHandler,
    ) -> ResolveResult<Option<(Identifier, Symbol)>> {
        match self {
            Statement::Workbench(wd) => Ok(Some((wd.id.clone(), wd.symbolize(parent, diag)?))),
            Statement::Module(md) => Ok(Some((md.id.clone(), md.symbolize(parent, diag)?))),
            Statement::Function(fd) => Ok(Some((fd.id.clone(), fd.symbolize(parent, diag)?))),
            Statement::Use(us) => us.symbolize(parent, diag),
            Statement::Assignment(a) => Ok(a
                .symbolize(parent, diag)?
                .map(|symbol| (a.assignment.id.clone(), symbol))),
            // Not producing any symbols
            Statement::Init(_)
            | Statement::Return(_)
            | Statement::If(_)
            | Statement::InnerAttribute(_)
            | Statement::Expression(_) => Ok(None),
        }
    }
}

impl Resolve<Symbol> for WorkbenchDefinition {
    fn symbolize(&self, parent: &Symbol, diag: &DiagHandler) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(
            SymbolDefinition::Workbench(self.clone().into()),
            Some(parent.clone()),
        );
        symbol.set_children(self.body.symbolize(&symbol, diag)?);
        Ok(symbol)
    }
}

impl Resolve<Symbol> for FunctionDefinition {
    fn symbolize(&self, parent: &Symbol, diag: &DiagHandler) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(
            SymbolDefinition::Function((*self).clone().into()),
            Some(parent.clone()),
        );
        symbol.set_children(self.body.symbolize(&symbol, diag)?);

        Ok(symbol)
    }
}

impl Resolve for InitDefinition {
    fn symbolize(&self, _parent: &Symbol, diag: &DiagHandler) -> ResolveResult<Option<Symbol>> {
        Ok(None)
    }
}

impl Resolve<Option<(Identifier, Symbol)>> for UseStatement {
    fn symbolize(
        &self,
        parent: &Symbol,
        diag: &DiagHandler,
    ) -> ResolveResult<Option<(Identifier, Symbol)>> {
        match self.visibility {
            Visibility::Private => Ok(None),
            // Public symbols are put into resolving symbol map
            Visibility::Public => self.decl.symbolize(parent, diag),
        }
    }
}

impl Resolve for ReturnStatement {
    fn symbolize(&self, _parent: &Symbol, diag: &DiagHandler) -> ResolveResult<Option<Symbol>> {
        Ok(None)
    }
}

impl Resolve for IfStatement {
    fn symbolize(&self, _parent: &Symbol, diag: &DiagHandler) -> ResolveResult<Option<Symbol>> {
        Ok(None)
    }
}

impl Resolve for Attribute {
    fn symbolize(&self, _parent: &Symbol, diag: &DiagHandler) -> ResolveResult<Option<Symbol>> {
        Ok(None)
    }
}

impl Resolve for AssignmentStatement {
    fn symbolize(&self, parent: &Symbol, diag: &DiagHandler) -> ResolveResult<Option<Symbol>> {
        match (self.assignment.visibility, self.assignment.qualifier) {
            // properties do not have a visibility
            (_, Qualifier::Prop) => {
                if !parent.can_prop() {
                    Err(ResolveError::DeclNotAllowed(
                        self.assignment.id.clone(),
                        parent.full_name(),
                    ))
                } else {
                    Ok(None)
                }
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
}

impl Resolve for ExpressionStatement {
    fn symbolize(&self, _parent: &Symbol, diag: &DiagHandler) -> ResolveResult<Option<Symbol>> {
        Ok(None)
    }
}

impl Resolve<SymbolMap> for Body {
    fn symbolize(&self, parent: &Symbol, diag: &DiagHandler) -> ResolveResult<SymbolMap> {
        self.statements.symbolize(parent, diag)
    }
}

impl Resolve<Option<(Identifier, Symbol)>> for UseDeclaration {
    fn symbolize(
        &self,
        parent: &Symbol,
        diag: &DiagHandler,
    ) -> ResolveResult<Option<(Identifier, Symbol)>> {
        match self {
            UseDeclaration::Use(visibility, name) => {
                let identifier = name.last().expect("Identifier");
                Ok(Some((
                    identifier.clone(),
                    Symbol::new(
                        SymbolDefinition::Alias(*visibility, identifier.clone(), name.clone()),
                        Some(parent.clone()),
                    ),
                )))
            }
            UseDeclaration::UseAll(_visibility, _) => Ok(None),
            UseDeclaration::UseAlias(visibility, name, alias) => Ok(Some((
                alias.clone(),
                Symbol::new(
                    SymbolDefinition::Alias(*visibility, alias.clone(), name.clone()),
                    Some(parent.clone()),
                ),
            ))),
        }
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

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

pub use externals::*;
pub use resolve_error::*;
pub use sources::*;
pub use symbol::*;
pub use symbol_definition::*;
pub use symbol_map::*;

use crate::{syntax::*, value::Value};

/// Trait for items which can be fully qualified.
pub trait FullyQualify {
    /// Get a fully (up to root of symbol map) qualified name.
    fn full_name(&self) -> QualifiedName;
}

trait Resolve<T = Option<Symbol>> {
    /// Resolve into Symbol
    fn resolve(&self, parent: &Symbol) -> ResolveResult<T>;

    //    fn resolve2(&self, parent: &Symbol
}

#[test]
fn resolve_test() {
    let root = SourceFile::load("../examples/lego_brick.µcad").expect("loading failed");
    let sources = Sources::load(root.clone(), &["../lib".into()]).expect("loading failed");
    let root_id = sources.root().id();
    let symbols = sources.resolve().expect("resolve failed");
    log::trace!("Symbols:\n{symbols}");

    crate::eval::SymbolTable::new(root_id, symbols, sources).expect("new symbol table");
}

impl SourceFile {
    /// Resolve into Symbol
    pub fn resolve(&self) -> ResolveResult<Symbol> {
        let name = self.filename_as_str();
        log::debug!("Creating symbol from source file {name}");
        let symbol = Symbol::new(SymbolDefinition::SourceFile(self.clone().into()), None);
        symbol.borrow_mut().children = self.statements.resolve(&symbol)?;
        log::trace!("Created symbol from source file {name}:\n{symbol}");
        Ok(symbol)
    }
}

impl Resolve<Symbol> for ModuleDefinition {
    /// Resolve into Symbol
    fn resolve(&self, parent: &Symbol) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(
            SymbolDefinition::Module(self.clone().into()),
            Some(parent.clone()),
        );
        symbol.borrow_mut().children = self.body.resolve(&symbol)?;
        Ok(symbol)
    }
}

impl Resolve<SymbolMap> for StatementList {
    fn resolve(&self, parent: &Symbol) -> ResolveResult<SymbolMap> {
        let mut symbols = SymbolMap::default();

        // Iterate over all statement fetch definitions
        for statement in &self.0 {
            if let Some((id, symbol)) = statement.resolve(parent)? {
                symbols.insert(id, symbol);
            }
        }

        Ok(symbols)
    }
}

impl Resolve<Option<(Identifier, Symbol)>> for Statement {
    fn resolve(&self, parent: &Symbol) -> ResolveResult<Option<(Identifier, Symbol)>> {
        match self {
            Statement::Workbench(wd) => Ok(Some((wd.id.clone(), wd.resolve(parent)?))),
            Statement::Module(md) => Ok(Some((md.id.clone(), md.resolve(parent)?))),
            Statement::Function(fd) => Ok(Some((fd.id.clone(), fd.resolve(parent)?))),
            Statement::Use(us) => us.resolve(parent),
            Statement::Assignment(a) => Ok(a
                .resolve(parent)?
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
    fn resolve(&self, parent: &Symbol) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(
            SymbolDefinition::Workbench(self.clone().into()),
            Some(parent.clone()),
        );
        symbol.borrow_mut().children = self.body.resolve(&symbol)?;
        Ok(symbol)
    }
}

impl Resolve<Symbol> for FunctionDefinition {
    fn resolve(&self, parent: &Symbol) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(
            SymbolDefinition::Function((*self).clone().into()),
            Some(parent.clone()),
        );
        symbol.borrow_mut().children = self.body.resolve(&symbol)?;

        Ok(symbol)
    }
}

impl Resolve for InitDefinition {
    fn resolve(&self, _parent: &Symbol) -> ResolveResult<Option<Symbol>> {
        Ok(None)
    }
}

impl Resolve<Option<(Identifier, Symbol)>> for UseStatement {
    fn resolve(&self, parent: &Symbol) -> ResolveResult<Option<(Identifier, Symbol)>> {
        match self.visibility {
            Visibility::Private => Ok(None),
            // Public symbols are put into resolving symbol map
            Visibility::Public => self.decl.resolve(parent),
        }
    }
}

impl Resolve for ReturnStatement {
    fn resolve(&self, _parent: &Symbol) -> ResolveResult<Option<Symbol>> {
        Ok(None)
    }
}

impl Resolve for IfStatement {
    fn resolve(&self, _parent: &Symbol) -> ResolveResult<Option<Symbol>> {
        Ok(None)
    }
}

impl Resolve for Attribute {
    fn resolve(&self, _parent: &Symbol) -> ResolveResult<Option<Symbol>> {
        Ok(None)
    }
}

impl Resolve for AssignmentStatement {
    fn resolve(&self, parent: &Symbol) -> ResolveResult<Option<Symbol>> {
        match self.assignment.qualifier {
            Qualifier::Prop => Ok(None),
            Qualifier::Const => todo!("create symbol"),
            Qualifier::Value => {
                log::trace!("Declare value {}", self.assignment.id);
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
    }
}

impl Resolve for ExpressionStatement {
    fn resolve(&self, _parent: &Symbol) -> ResolveResult<Option<Symbol>> {
        Ok(None)
    }
}

impl Resolve<SymbolMap> for Body {
    fn resolve(&self, parent: &Symbol) -> ResolveResult<SymbolMap> {
        self.statements.resolve(parent)
    }
}

impl Resolve<Option<(Identifier, Symbol)>> for UseDeclaration {
    fn resolve(&self, parent: &Symbol) -> ResolveResult<Option<(Identifier, Symbol)>> {
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
            UseDeclaration::UseAll(visibility, _) => Ok(None),
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
    let source_file =
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
}

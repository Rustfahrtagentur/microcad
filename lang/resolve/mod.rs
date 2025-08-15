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
//! let source_symbol = source_file.resolve(None);
//! ```
//!
//! To "run" the source file (and get the expected output) it must now be evaluated (see [`crate::eval`])  .

mod externals;
mod resolve_error;
mod source_cache;
mod symbol;
mod symbol_definition;
mod symbol_map;

pub use externals::*;
pub use resolve_error::*;
pub use source_cache::*;
pub use symbol::*;
pub use symbol_definition::*;
pub use symbol_map::*;

use crate::{diag::*, syntax::*};

/// Trait for items which can be fully qualified.
pub trait FullyQualify {
    /// Get a fully (up to root of symbol map) qualified name.
    fn full_name(&self) -> QualifiedName;
}

#[derive(Default)]
pub struct ResolveContext {
    /// List of all global symbols.
    pub symbols: SymbolMap,
    /// Source file cache containing all source files loaded in the context and their syntax trees.
    pub cache: SourceCache,
    /// Source file diagnostics.
    pub diag_handler: DiagHandler,
}

trait Resolve<T = Option<Symbol>> {
    /// Resolve into Symbol
    fn resolve(&self, parent: &Symbol) -> ResolveResult<T>;
}

impl ResolveContext {
    /// Load a symbol from a qualified name.
    ///
    /// Might load any related external file if not already loaded.
    ///
    /// # Arguments
    /// - `name`: Name of the symbol to load
    pub fn load_symbol(&mut self, name: &QualifiedName) -> ResolveResult<Symbol> {
        log::trace!("Trying to load symbol {name}");

        // if symbol could not be found in symbol tree, try to load it from external file
        match self.cache.get_by_name(name) {
            Err(ResolveError::SymbolMustBeLoaded(_, path)) => {
                log::trace!(
                    "{load} symbol {name} from {path:?}",
                    load = crate::mark!(LOAD)
                );
                let source_file =
                    SourceFile::load_with_name(path.clone(), self.cache.name_by_path(&path)?)?;
                let source_name = self.cache.insert(source_file.clone())?;
                let node = source_file.resolve(None)?;
                // search module where to place loaded source file into
                let target = self.symbols.search(&source_name)?;
                Symbol::move_children(&target, &node);
                // mark target as "loaded" by changing the SymbolDefinition type
                target.external_to_module();
            }
            Ok(_) => (),
            Err(ResolveError::SymbolNotFound(_)) => {
                return Err(ResolveError::SymbolNotFound(name.clone()))
            }
            Err(err) => return Err(err)?,
        }

        // get symbol from symbol map
        self.symbols.search(name)
    }

    /// Lookup a symbol from global symbols.
    pub fn lookup(&mut self, name: &QualifiedName) -> ResolveResult<Symbol> {
        log::trace!("Looking for global symbol '{name}'");
        let symbol = match self.symbols.search(name) {
            Ok(symbol) => symbol.clone(),
            Err(ResolveError::SymbolNotFound(_)) => self.load_symbol(name)?,
            Err(err) => return Err(err)?,
        };
        log::trace!(
            "{found} global symbol '{name}': = '{full_name}'",
            found = crate::mark!(FOUND),
            full_name = symbol.full_name()
        );
        Ok(symbol)
    }
}

impl SourceFile {
    /// Resolve into Symbol
    pub fn resolve(&self, parent: Option<Symbol>) -> ResolveResult<Symbol> {
        let name = self.filename_as_str();
        log::debug!("Resolving source file {name}");
        let symbol = Symbol::new(SymbolDefinition::SourceFile(self.clone().into()), parent);
        symbol.borrow_mut().children = self.statements.resolve(&symbol)?;
        log::trace!("Resolved source file {name}:\n{symbol}");
        Ok(symbol)
    }
}

impl Resolve<Symbol> for ModuleDefinition {
    /// Resolve into Symbol
    fn resolve(&self, parent: &Symbol) -> ResolveResult<Symbol> {
        let node = Symbol::new(
            SymbolDefinition::Module(self.clone().into()),
            Some(parent.clone()),
        );
        node.borrow_mut().children = self.body.resolve(&node)?;
        Ok(node)
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
            // Not producing any symbols
            Statement::Init(_)
            | Statement::Return(_)
            | Statement::If(_)
            | Statement::InnerAttribute(_)
            | Statement::Assignment(_)
            | Statement::Expression(_) => Ok(None),
        }
    }
}

impl Resolve<Symbol> for WorkbenchDefinition {
    fn resolve(&self, parent: &Symbol) -> ResolveResult<Symbol> {
        let node = Symbol::new(
            SymbolDefinition::Workbench(self.clone().into()),
            Some(parent.clone()),
        );
        node.borrow_mut().children = self.body.resolve(&node)?;
        Ok(node)
    }
}

impl Resolve<Symbol> for FunctionDefinition {
    fn resolve(&self, parent: &Symbol) -> ResolveResult<Symbol> {
        let node = Symbol::new(
            SymbolDefinition::Function((*self).clone().into()),
            Some(parent.clone()),
        );
        node.borrow_mut().children = self.body.resolve(&node)?;
        Ok(node)
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
    fn resolve(&self, _parent: &Symbol) -> ResolveResult<Option<Symbol>> {
        Ok(None)
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
            UseDeclaration::Use(name) => {
                let identifier = name.last().expect("Identifier");
                Ok(Some((
                    identifier.clone(),
                    Symbol::new(
                        SymbolDefinition::Alias(identifier.clone(), name.clone()),
                        Some(parent.clone()),
                    ),
                )))
            }
            UseDeclaration::UseAll(_) => Ok(None),
            UseDeclaration::UseAlias(name, alias) => Ok(Some((
                alias.clone(),
                Symbol::new(
                    SymbolDefinition::Alias(alias.clone(), name.clone()),
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

    let symbol_node = source_file
        .resolve(None)
        .expect("expecting resolve success");

    // file <no file>
    //  part a
    //   part b
    assert!(symbol_node.get(&"A".into()).is_some());
    assert!(symbol_node.get(&"c".into()).is_none());

    assert!(symbol_node.search(&"A".into()).is_some());
    assert!(symbol_node.search(&"A::B".into()).is_some());
    assert!(symbol_node.search(&"A::B::C".into()).is_none());

    // use std::print; // Add symbol "print" to current symbol node
    // part M() {
    //      print("test"); // Use symbol node from parent
    // }

    log::trace!("Symbol node:\n{symbol_node}");

    let b = symbol_node.search(&"A::B".into()).expect("cant find node");
    assert!(b.search(&"A".into()).is_none());

    //assert!(symbol_node.search_top_down(&["<no file>".into()]).is_some());

    log::trace!("{symbol_node}");
}

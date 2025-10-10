// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Symbolizing of syntax definitions.

use crate::{diag::*, resolve::*, syntax::*};

pub(super) trait Symbolize<T = Option<Symbol>> {
    /// Create symbol from definition.
    fn symbolize(&self, _parent: &Symbol, _context: &mut ResolveContext) -> ResolveResult<T> {
        unreachable!()
    }
}

impl SourceFile {
    /// Create symbol from definition.
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
                .grant(&symbol, context)?
                .symbolize(&symbol, context)?,
        );
        Ok(symbol)
    }
}

impl Symbolize<Symbol> for ModuleDefinition {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<Symbol> {
        let symbol = if let Some(body) = &self.body {
            let symbol = Symbol::new(
                SymbolDefinition::Module(self.clone().into()),
                Some(parent.clone()),
            );
            symbol.set_children(body.grant(&symbol, context)?.symbolize(&symbol, context)?);
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
}

impl Symbolize<SymbolMap> for StatementList {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<SymbolMap> {
        let mut symbols = SymbolMap::default();

        // Iterate over all statement fetch definitions
        for statement in &self.0 {
            if let Some((id, symbol)) = statement
                .grant(parent, context)?
                .symbolize(parent, context)?
            {
                symbols.insert(id, symbol);
            }
        }

        Ok(symbols)
    }
}

impl Symbolize<Option<(Identifier, Symbol)>> for Statement {
    fn symbolize(
        &self,
        parent: &Symbol,
        context: &mut ResolveContext,
    ) -> ResolveResult<Option<(Identifier, Symbol)>> {
        match self {
            Statement::Workbench(wd) => Ok(Some((
                wd.id.clone(),
                wd.grant(parent, context)?.symbolize(parent, context)?,
            ))),
            Statement::Module(md) => Ok(Some((
                md.id.clone(),
                md.grant(parent, context)?.symbolize(parent, context)?,
            ))),
            Statement::Function(fd) => Ok(Some((
                fd.id.clone(),
                fd.grant(parent, context)?.symbolize(parent, context)?,
            ))),
            Statement::Use(us) => us.grant(parent, context)?.symbolize(parent, context),
            Statement::Assignment(a) => Ok(a
                .grant(parent, context)?
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
}

impl Symbolize<Symbol> for WorkbenchDefinition {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(
            SymbolDefinition::Workbench(self.clone().into()),
            Some(parent.clone()),
        );
        symbol.set_children(
            self.body
                .grant(&symbol, context)?
                .symbolize(&symbol, context)?,
        );
        Ok(symbol)
    }
}

impl Symbolize<Symbol> for FunctionDefinition {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<Symbol> {
        let symbol = Symbol::new(
            SymbolDefinition::Function((*self).clone().into()),
            Some(parent.clone()),
        );
        symbol.set_children(
            self.body
                .grant(&symbol, context)?
                .symbolize(&symbol, context)?,
        );

        Ok(symbol)
    }
}

impl Symbolize for AssignmentStatement {
    fn symbolize(
        &self,
        parent: &Symbol,
        context: &mut ResolveContext,
    ) -> ResolveResult<Option<Symbol>> {
        let symbol = match (self.assignment.visibility, self.assignment.qualifier()) {
            // properties do not have a visibility
            (_, Qualifier::Prop) => {
                if !parent.can_prop() {
                    None
                } else {
                    Some(None)
                }
            }
            // constants will be symbols (`pub` shall equal `pub const`)
            (_, Qualifier::Const) | (Visibility::Public, Qualifier::Value) => {
                if !parent.can_const() {
                    None
                } else {
                    log::trace!("Declaring private const expression: {}", self.assignment.id);
                    Some(Some(Symbol::new(
                        SymbolDefinition::ConstExpression(
                            self.assignment.visibility,
                            self.assignment.id.clone(),
                            self.assignment.expression.clone(),
                        ),
                        Some(parent.clone()),
                    )))
                }
            }
            // value go on stack
            (Visibility::Private, Qualifier::Value) => {
                if self.assignment.visibility == Visibility::Private && !parent.can_value() {
                    None
                } else {
                    Some(None)
                }
            }
            (Visibility::Deleted, _) => unreachable!(),
        };

        match symbol {
            Some(symbol) => Ok(symbol),
            None => {
                context.error(
                    self,
                    ResolveError::DeclNotAllowed(self.assignment.id.clone(), parent.full_name()),
                )?;
                Ok(None)
            }
        }
    }
}

impl Symbolize<SymbolMap> for Body {
    fn symbolize(&self, parent: &Symbol, context: &mut ResolveContext) -> ResolveResult<SymbolMap> {
        self.statements
            .grant(parent, context)?
            .symbolize(parent, context)
    }
}

impl Symbolize<Option<(Identifier, Symbol)>> for UseStatement {
    fn symbolize(
        &self,
        parent: &Symbol,
        _: &mut ResolveContext,
    ) -> ResolveResult<Option<(Identifier, Symbol)>> {
        if !parent.is_module() {
            return Ok(None);
        }
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
}

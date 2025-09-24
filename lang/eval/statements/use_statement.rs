// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

/// Trait used by [UseDeclaration] and implemented by [SymbolTable] and passed through by [Context]
/// to put symbols on the [Stack] (for `use statements`).
pub trait UseSymbol {
    /// Find a symbol in the symbol table and copy it to the locals.
    ///
    /// Might load any related external file if not already loaded.
    ///
    /// # Arguments
    /// - `name`: Name of the symbol to search for
    /// - `id`: if given overwrites the ID from qualified name (use as)
    /// - `within`: Target symbol
    fn use_symbol(
        &mut self,
        visibility: Visibility,
        name: &QualifiedName,
        id: Option<Identifier>,
        within: &QualifiedName,
    ) -> EvalResult<Symbol>;

    /// Find a symbol in the symbol table and copy all it's children to the locals and the target.
    ///
    /// Might load any related external file if not already loaded.
    ///
    /// # Arguments
    /// - `name`: Name of the symbol to search for
    /// - `within`: Target symbol
    fn use_symbols_of(
        &mut self,
        visibility: Visibility,
        name: &QualifiedName,
        within: &QualifiedName,
    ) -> EvalResult<Symbol>;
}

impl Eval<()> for UseStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<()> {
        context.grant(self)?;
        log::debug!("Evaluating use statement: {self}");
        if matches!(self.decl, UseDeclaration::UseAll(..)) || self.visibility == Visibility::Private
        {
            self.decl.eval(context)
        } else {
            Ok(())
        }
    }
}

impl Eval<()> for UseDeclaration {
    fn eval(&self, context: &mut Context) -> EvalResult<()> {
        todo!()
        /*
        match &self {
            UseDeclaration::Use(visibility, name) => {
                if let Err(err) = context.use_symbol(*visibility, name, None) {
                    context.error(name, err)?;
                }
            }
            UseDeclaration::UseAll(visibility, name) => {
                if let Err(err) = context.use_symbols_of(*visibility, name) {
                    context.error(name, err)?
                }
            }
            UseDeclaration::UseAlias(visibility, name, alias) => {
                if let Err(err) = context.use_symbol(*visibility, name, Some(alias.clone())) {
                    context.error(name, err)?;
                }
            }
        };
        Ok(())
        */
    }
}

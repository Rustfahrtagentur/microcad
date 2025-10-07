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
    fn eval(&self, context: &mut EvalContext) -> EvalResult<()> {
        context.grant(self)?;

        if !context.is_module() {
            log::trace!("Evaluating use statement: {self}");
            let current = &context.current_name();
            let visibility = self.visibility;
            match &self.decl {
                UseDeclaration::Use(name) => {
                    if let Err(err) = context.use_symbol(visibility, name, None, current) {
                        context.error(name, err)?;
                    }
                }
                UseDeclaration::UseAll(name) => {
                    if let Err(err) = context.use_symbols_of(visibility, name, current) {
                        context.error(name, err)?
                    }
                }
                UseDeclaration::UseAlias(name, alias) => {
                    if let Err(err) =
                        context.use_symbol(visibility, name, Some(alias.clone()), current)
                    {
                        context.error(name, err)?;
                    }
                }
            }
        }
        Ok(())
    }
}

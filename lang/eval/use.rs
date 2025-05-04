// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

/// trait used by UseDeclaration and implemented by SymbolTable and passed through by EvalContext
pub trait UseSymbol {
    /// Find a symbol in the symbol table and copy it to the locals.
    ///
    /// Might load any related external file if not already loaded.
    ///
    /// # Arguments
    /// - `name`: Name of the symbol to search for
    /// - `id`: if given overwrites the ID from qualified name (use as)
    fn use_symbol(
        &mut self,
        name: &QualifiedName,
        id: Option<Identifier>,
    ) -> EvalResult<SymbolNode>;

    /// Find a symbol in the symbol table and copy all it's children to the locals.
    ///
    /// Might load any related external file if not already loaded.
    ///
    /// # Arguments
    /// - `name`: Name of the symbol to search for
    fn use_symbols_of(&mut self, name: &QualifiedName) -> EvalResult<SymbolNode>;
}

impl Eval for UseStatement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        self.decl.eval(context)
    }
}

impl Eval for UseDeclaration {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match &self {
            UseDeclaration::Use(name) => {
                if let Err(err) = context.use_symbol(name, None) {
                    context.error(name, err)?;
                }
            }
            UseDeclaration::UseAll(name) => {
                if let Err(err) = context.use_symbols_of(name) {
                    context.error(name, err)?
                }
            }
            UseDeclaration::UseAlias(name, alias) => {
                if let Err(err) = context.use_symbol(name, Some(alias.clone())) {
                    context.error(name, err)?;
                }
            }
        };
        Ok(Value::None)
    }
}

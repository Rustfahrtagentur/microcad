// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module body parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};
use microcad_render::tree;

/// Module body
#[derive(Clone, Debug, Default)]
pub struct ModuleBody {
    /// Module statements
    pub statements: Vec<ModuleStatement>,
    /// Module's local symbol table
    pub symbols: SymbolTable,
    /// Initializers
    pub inits: Vec<std::rc::Rc<ModuleInitDefinition>>,
    /// Source code reference
    src_ref: SrcRef,
}

impl ModuleBody {
    /// Add statement to module
    pub fn add_statement(&mut self, statement: ModuleStatement) {
        self.statements.push(statement.clone());
        match statement {
            ModuleStatement::FunctionDefinition(function) => {
                self.add_function(function);
            }
            ModuleStatement::ModuleDefinition(module) => {
                self.add_module(module);
            }
            ModuleStatement::ModuleInitDefinition(init) => {
                self.inits.push(init.clone());
            }
            _ => {}
        }
    }
}

impl SrcReferrer for ModuleBody {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Symbols for ModuleBody {
    fn fetch_symbols(&self, id: &Id) -> Vec<&Symbol> {
        self.symbols.fetch_symbols(id)
    }

    fn add_symbol(&mut self, symbol: Symbol) -> &mut Self {
        self.symbols.add_symbol(symbol);
        self
    }

    fn copy_symbols<T: Symbols>(&self, into: &mut T) {
        self.symbols.copy_symbols(into)
    }
}

impl Parse for ModuleBody {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_body);
        let mut body = ModuleBody::default();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::module_statement => {
                    let statement = ModuleStatement::parse(pair.clone())?;
                    body.add_statement(statement);
                }
                Rule::expression => {
                    let expression = Expression::parse(pair.clone())?;
                    body.add_statement(ModuleStatement::Expression(expression));
                }
                _ => {}
            }
        }

        body.src_ref = pair.into();

        Ok(body)
    }
}

impl Eval for ModuleBody {
    type Output = tree::Node;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let node = tree::group();
        let current = context.current_node();
        context.set_current_node(node.clone());
        for statement in &self.statements {
            statement.eval(context)?;
        }
        context.set_current_node(current.clone());

        Ok(node.clone())
    }
}

impl std::fmt::Display for ModuleBody {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, " {{")?;
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}


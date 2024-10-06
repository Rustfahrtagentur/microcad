// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module body parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};
use microcad_render::tree;

/// Module body
#[derive(Clone, Debug, Default)]
pub struct ModuleBody {
    /// Module statements before init
    pub pre_init_statements: Vec<ModuleStatement>,
    /// Module statements after init
    pub post_init_statements: Vec<ModuleStatement>,
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
    pub fn add_statement(&mut self, statement: ModuleStatement) -> ParseResult<()> {
        self.statements.push(statement.clone());
        match statement {
            ModuleStatement::FunctionDefinition(function) => {
                self.add(function.into());
            }
            ModuleStatement::ModuleDefinition(module) => {
                self.add(module.into());
            }
            ModuleStatement::ModuleInitDefinition(init) => {
                // Initializers are only allowed after pre-init statements
                // and before post-init statements.
                // Other statements between pre-init and post-init are not allowed
                if self.post_init_statements.is_empty() {
                    self.inits.push(init.clone());
                } else {
                    return Err(ParseError::StatementBetweenModuleInit);
                }

                self.inits.push(init.clone());
            }
            _ => {}
        }

        Ok(())
    }

    /// Add default initializer to body
    /// If a body has no initializer, but the module definition has parameters,
    /// this function will add a default initializer to the body
    pub fn add_initializer_from_parameter_list(
        &mut self,
        parameters: ParameterList,
    ) -> ParseResult<()> {
        if !self.inits.is_empty() {
            return Err(ParseError::BothParameterListAndInitializer);
        }

        let mut init = ModuleInitDefinition::new(parameters, Vec::new());
        init.src_ref = init.parameters.src_ref();
        self.inits.push(std::rc::Rc::new(init));

        // Move pre-init statements to post-init statements
        std::mem::swap(
            &mut self.pre_init_statements,
            &mut self.post_init_statements,
        );

        Ok(())
    }
}

impl SrcReferrer for ModuleBody {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Symbols for ModuleBody {
    fn fetch(&self, id: &Id) -> Vec<&Symbol> {
        self.symbols.fetch(id)
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.symbols.add(symbol);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) {
        self.symbols.copy(into)
    }
}

impl Parse for ModuleBody {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_body);
        let mut body = ModuleBody::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::module_statement => {
                    let statement = ModuleStatement::parse(pair.clone())?;
                    body.add_statement(statement)?;
                }
                Rule::expression => {
                    let expression = Expression::parse(pair.clone())?;
                    body.add_statement(ModuleStatement::Expression(expression))?;
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

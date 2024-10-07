// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module body parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Module definition body
///
/// An example for a module definition body:
///
/// ```microCAD
/// module donut {
///     a = 2; // Pre-init statement
///
///     init(d: length) { // init definition
///         radius = d / 2;
///     }
///
///     init(r: length) { // Another init definition
///
///     }
///
///     b = 2; // Post-init statement
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct ModuleDefinitionBody {
    /// Module statements before init
    pub pre_init_statements: Vec<ModuleDefinitionStatement>,
    /// Module statements after init
    pub post_init_statements: Vec<ModuleDefinitionStatement>,
    /// Module's local symbol table
    pub symbols: SymbolTable,
    /// Initializers
    pub inits: Vec<std::rc::Rc<ModuleInitDefinition>>,
    /// Source code reference
    src_ref: SrcRef,
}

impl ModuleDefinitionBody {
    /// Add statement to module
    pub fn add_statement(&mut self, statement: ModuleDefinitionStatement) -> ParseResult<()> {
        match statement {
            ModuleDefinitionStatement::FunctionDefinition(function) => {
                self.add(function.into());
            }
            ModuleDefinitionStatement::ModuleDefinition(module) => {
                self.add(module.into());
            }
            ModuleDefinitionStatement::ModuleInitDefinition(init) => {
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
            statement => {
                if self.inits.is_empty() {
                    self.pre_init_statements.push(statement);
                } else {
                    self.post_init_statements.push(statement);
                }
            }
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

        let src_ref = parameters.src_ref();
        let init = ModuleInitDefinition {
            parameters,
            body: NodeBody::default(),
            src_ref,
        };
        self.inits.push(std::rc::Rc::new(init));

        // Move pre-init statements to post-init statements
        std::mem::swap(
            &mut self.pre_init_statements,
            &mut self.post_init_statements,
        );

        Ok(())
    }
}

impl SrcReferrer for ModuleDefinitionBody {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Symbols for ModuleDefinitionBody {
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

impl Parse for ModuleDefinitionBody {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::module_definition_body);
        let mut body = Self::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::module_definition_statement => {
                    let statement = ModuleDefinitionStatement::parse(pair.clone())?;
                    body.add_statement(statement)?;
                }
                Rule::expression => {
                    let expression = Expression::parse(pair.clone())?;
                    body.add_statement(ModuleDefinitionStatement::Expression(expression))?;
                }
                _ => {}
            }
        }

        body.src_ref = pair.into();

        Ok(body)
    }
}

impl std::fmt::Display for ModuleDefinitionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, " {{")?;
        for statement in &self.pre_init_statements {
            writeln!(f, "{}", statement)?;
        }

        for init in &self.inits {
            writeln!(f, "{}", init)?;
        }

        for statement in &self.post_init_statements {
            writeln!(f, "{}", statement)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

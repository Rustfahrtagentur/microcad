// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module body parser entity

use crate::{eval::*, objects::*, parse::*, parser::*, src_ref::*, sym::*};

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
    /// implicit Initializers
    pub implicit_init: Option<std::rc::Rc<ModuleInitDefinition>>,
    /// explicit Initializers
    pub explicit_inits: Vec<std::rc::Rc<ModuleInitDefinition>>,
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
                    self.explicit_inits.push(init.clone());
                } else {
                    return Err(ParseError::StatementBetweenModuleInit);
                }
            }
            statement => {
                if self.explicit_inits.is_empty() {
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
        let src_ref = parameters.src_ref();
        let init = ModuleInitDefinition {
            parameters,
            body: NodeBody::default(),
            src_ref,
        };
        self.implicit_init = Some(std::rc::Rc::new(init));

        Ok(())
    }

    /// Evaluate a single statement of the module
    fn eval_statement(
        &self,
        statement: &ModuleDefinitionStatement,
        context: &mut EvalContext,
        group: &mut ObjectNode,
    ) -> EvalResult<()> {
        match statement {
            ModuleDefinitionStatement::Assignment(assignment) => {
                // Evaluate the assignment and add the symbol to the node
                // E.g. `a = 1` will add the symbol `a` to the node
                let symbol = assignment.eval(context)?;
                group.add(symbol);
            }
            ModuleDefinitionStatement::FunctionDefinition(function) => {
                // Evaluate the function and add the symbol to the node
                // E.g. `function a() {}` will add the symbol `a` to the node
                let symbol = function.eval(context)?;
                group.add(symbol);
            }
            statement => {
                if let Some(Value::Node(new_child)) = statement.eval(context)? {
                    group.append(new_child);
                }
            }
        }
        Ok(())
    }

    /// Evaluate the pre-init statements, and copy the symbols to the node
    pub fn eval_pre_init_statements(
        &self,
        context: &mut EvalContext,
        node: &mut ObjectNode,
    ) -> EvalResult<()> {
        for statement in &self.pre_init_statements {
            self.eval_statement(statement, context, node)?;
        }
        node.copy(context)?;
        Ok(())
    }

    /// Evaluate the post-init statements, and copy the symbols to the node
    pub fn eval_post_init_statements(
        &self,
        context: &mut EvalContext,
        node: &mut ObjectNode,
    ) -> EvalResult<()> {
        for statement in &self.post_init_statements {
            self.eval_statement(statement, context, node)?;
        }
        node.copy(context)?;
        Ok(())
    }
}

impl SrcReferrer for ModuleDefinitionBody {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Symbols for ModuleDefinitionBody {
    fn fetch(&self, id: &Id) -> Option<std::rc::Rc<Symbol>> {
        self.symbols.fetch(id)
    }

    fn add(&mut self, symbol: Symbol) -> &mut Self {
        self.symbols.add(symbol);
        self
    }

    fn add_alias(&mut self, symbol: Symbol, alias: Id) -> &mut Self {
        self.symbols.add_alias(symbol, alias);
        self
    }

    fn copy<T: Symbols>(&self, into: &mut T) -> SymResult<()> {
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
        if let Some(init) = &self.implicit_init {
            writeln!(f, "{}", init)?;
        }

        for statement in &self.pre_init_statements {
            writeln!(f, "{}", statement)?;
        }

        for init in &self.explicit_inits {
            writeln!(f, "{}", init)?;
        }

        for statement in &self.post_init_statements {
            writeln!(f, "{}", statement)?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

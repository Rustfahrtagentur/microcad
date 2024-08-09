// Resolve a qualified name to a type or value.

use std::fmt::Display;
use std::rc::Rc;

use crate::call::CallArgumentList;
use crate::eval::Symbol;
use crate::expression::Expression;
use crate::function::{Assignment, DefinitionParameter, FunctionDefinition};
use crate::identifier::{Identifier, QualifiedName};
use crate::{parser::*, with_pair_ok};

#[derive(Clone)]
pub struct Attribute {
    pub name: QualifiedName,
    pub arguments: Option<CallArgumentList>,
}

impl Parse for Attribute {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        let mut pairs = pair.into_inner();
        let name = QualifiedName::parse(pairs.next().unwrap())?.value().clone();

        if let Some(p) = pairs.next() {
            with_pair_ok!(
                Attribute {
                    name,
                    arguments: Some(CallArgumentList::parse(p.clone())?.value().clone()),
                },
                p
            )
        } else {
            with_pair_ok!(
                Attribute {
                    name,
                    arguments: None,
                },
                p
            )
        }
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.arguments {
            Some(arguments) => write!(f, "{}({})", self.name, arguments),
            None => write!(f, "{}", self.name),
        }
    }
}

#[derive(Clone)]
pub enum ModuleInitStatement {
    Use(UseStatement),
    Expression(Expression),
    Assignment(Assignment),
    FunctionDefinition(FunctionDefinition),
}

impl Parse for ModuleInitStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        let mut pairs = pair.into_inner();
        let first = pairs.next().unwrap();

        let s = match first.as_rule() {
            Rule::use_statement => {
                ModuleInitStatement::Use(UseStatement::parse(first)?.value().clone())
            }
            Rule::expression => {
                ModuleInitStatement::Expression(Expression::parse(first)?.value().clone())
            }
            Rule::assignment => {
                ModuleInitStatement::Assignment(Assignment::parse(first)?.value().clone())
            }
            Rule::function_definition => ModuleInitStatement::FunctionDefinition(
                FunctionDefinition::parse(first)?.value().clone(),
            ),
            _ => unreachable!(),
        };

        with_pair_ok!(s, p)
    }
}

#[derive(Clone)]
pub struct ModuleInitDefinition {
    #[allow(dead_code)]
    parameters: Vec<DefinitionParameter>,
    #[allow(dead_code)]
    body: Vec<ModuleInitStatement>,
}

impl Parse for ModuleInitDefinition {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        let mut parameters = Vec::new();
        let mut body = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::definition_parameter_list => {
                    for pair in pair.into_inner() {
                        parameters.push(DefinitionParameter::parse(pair)?.value().clone());
                    }
                }
                Rule::module_init_statement => {
                    body.push(ModuleInitStatement::parse(pair)?.value().clone());
                }
                rule => unreachable!("expected definition_parameter_list or module_init_statement. Instead found {rule:?}" ),
            }
        }

        with_pair_ok!(ModuleInitDefinition { parameters, body }, p)
    }
}

#[derive(Clone)]
pub enum ModuleStatement {
    Use(UseStatement),
    Expression(Expression),
    Assignment(Assignment),
    ModuleDefinition(Rc<ModuleDefinition>),
    FunctionDefinition(Rc<FunctionDefinition>),
    ModuleInitDefinition(ModuleInitDefinition),
}

impl Parse for ModuleStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::module_statement);
        let p = pair.clone();
        let mut pairs = pair.into_inner();
        let first = pairs.next().unwrap();

        let s = match first.as_rule() {
            Rule::use_statement => {
                ModuleStatement::Use(UseStatement::parse(first)?.value().clone())
            }
            Rule::expression => {
                ModuleStatement::Expression(Expression::parse(first)?.value().clone())
            }
            Rule::assignment => {
                ModuleStatement::Assignment(Assignment::parse(first)?.value().clone())
            }
            Rule::module_definition => ModuleStatement::ModuleDefinition(Rc::new(
                ModuleDefinition::parse(first)?.value().clone(),
            )),
            Rule::module_init_definition => ModuleStatement::ModuleInitDefinition(
                ModuleInitDefinition::parse(first)?.value().clone(),
            ),
            Rule::function_definition => ModuleStatement::FunctionDefinition(Rc::new(
                FunctionDefinition::parse(first)?.value().clone(),
            )),
            rule => unreachable!("Unexpected module statement, got {:?}", rule),
        };

        with_pair_ok!(s, p)
    }
}

#[derive(Clone)]
pub struct ModuleDefinition {
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub parameters: Option<Vec<DefinitionParameter>>,
    pub body: Vec<ModuleStatement>,
}

impl ModuleDefinition {
    pub fn namespace(name: Identifier) -> Self {
        ModuleDefinition {
            attributes: Vec::new(),
            name,
            parameters: None,
            body: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: Rc<FunctionDefinition>) {
        self.body
            .push(ModuleStatement::FunctionDefinition(function));
    }

    pub fn add_module(&mut self, module: Rc<ModuleDefinition>) {
        self.body.push(ModuleStatement::ModuleDefinition(module));
    }

    pub fn get_symbol(&self, name: &Identifier) -> Option<Symbol> {
        for statement in &self.body {
            match statement {
                ModuleStatement::FunctionDefinition(function) => {
                    if &function.name == name {
                        return Some(crate::eval::Symbol::Function(function.clone()));
                    }
                }
                ModuleStatement::ModuleDefinition(module) => {
                    if &module.name == name {
                        return Some(crate::eval::Symbol::ModuleDefinition(module.clone()));
                    }
                }
                _ => {}
            }
        }
        None
    }
}

impl Parse for ModuleDefinition {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        let mut attributes = Vec::new();
        let mut name = Identifier::default();
        let mut parameters = None;
        let mut body = Vec::new();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::attribute_list => {
                    attributes.push(Attribute::parse(pair)?.value().clone());
                }
                Rule::identifier => {
                    name = Identifier::parse(pair)?.value().clone();
                }
                Rule::definition_parameter_list => {
                    parameters = Some(
                        Parser::vec(pair, DefinitionParameter::parse)?
                            .value()
                            .clone(),
                    );
                }
                Rule::module_body => {
                    for pair in pair.into_inner() {
                        if pair.as_rule() == Rule::module_statement {
                            body.push(ModuleStatement::parse(pair)?.value().clone());
                        }
                    }
                }
                rule => unreachable!("Unexpected module definition, got {:?}", rule),
            }
        }

        with_pair_ok!(
            ModuleDefinition {
                attributes,
                name,
                parameters,
                body,
            },
            p
        )
    }
}

#[derive(Clone)]
pub struct UseAlias(pub QualifiedName, pub Identifier);

impl std::fmt::Display for UseAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "use {:?} as {:?}", self.0, self.1)
    }
}

#[derive(Clone)]
pub enum UseStatement {
    /// Import symbols given as qualified names: `use a, b`
    Use(Vec<QualifiedName>),

    /// Import specific symbol from a module: `use a,b from c`
    UseFrom(Vec<QualifiedName>, QualifiedName),

    /// Import all symbols from a module: `use * from a, b`
    UseAll(Vec<QualifiedName>),

    /// Import as alias: `use a as b`
    UseAlias(UseAlias),
}

impl std::fmt::Display for UseStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UseStatement::Use(qualified_names) => write!(f, "use {:?}", qualified_names),
            UseStatement::UseFrom(qualified_names, from) => {
                write!(f, "use {:?} from {:?}", qualified_names, from)
            }
            UseStatement::UseAll(qualified_names) => write!(f, "use * from {:?}", qualified_names),
            UseStatement::UseAlias(alias) => write!(f, "{}", alias),
        }
    }
}

impl Parse for UseAlias {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        let mut pairs = pair.into_inner();
        with_pair_ok!(
            UseAlias(
                QualifiedName::parse(pairs.next().unwrap())?.value().clone(),
                Identifier::parse(pairs.next().unwrap())?.value().clone(),
            ),
            p
        )
    }
}

impl Parse for UseStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        let mut pairs = pair.into_inner();

        let first = pairs.next().unwrap();
        let second = pairs.next();

        match first.as_rule() {
            Rule::qualified_name_list => {
                let qualified_name_list = Parser::vec(first, QualifiedName::parse)?.value().clone();
                if let Some(second) = second {
                    if second.as_rule() == Rule::qualified_name {
                        return with_pair_ok!(
                            UseStatement::UseFrom(
                                qualified_name_list,
                                QualifiedName::parse(second)?.value().clone(),
                            ),
                            p
                        );
                    } else {
                        unreachable!();
                    }
                } else {
                    return with_pair_ok!(UseStatement::Use(qualified_name_list), p);
                }
            }
            Rule::qualified_name_all => {
                if let Some(second) = second {
                    if second.as_rule() == Rule::qualified_name_list {
                        return with_pair_ok!(
                            UseStatement::UseAll(
                                Parser::vec(second, QualifiedName::parse)?.value().clone()
                            ),
                            p
                        );
                    } else {
                        unreachable!();
                    }
                }
            }
            Rule::use_alias => {
                return with_pair_ok!(
                    UseStatement::UseAlias(UseAlias::parse(first)?.value().clone()),
                    p
                );
            }

            _ => unreachable!(),
        }

        Err(ParseError::InvalidUseStatement)
    }
}

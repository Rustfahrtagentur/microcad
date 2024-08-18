use microcad_render::tree::{self, Node};

use super::{
    assignment::*, call::*, expression::*, function::*, identifier::*, parameter::ParameterList,
    use_statement::*,
};
use crate::{eval::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: QualifiedName,
    pub arguments: Option<CallArgumentList>,
}

impl Parse for Attribute {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut inner = pair.clone().into_inner();
        let name = QualifiedName::parse(inner.next().unwrap())?.value().clone();
        match inner.next() {
            Some(pair) => with_pair_ok!(
                Attribute {
                    name,
                    arguments: Some(CallArgumentList::parse(pair.clone())?.value().clone()),
                },
                pair
            ),
            _ => with_pair_ok!(
                Attribute {
                    name,
                    arguments: None,
                },
                pair
            ),
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.arguments {
            Some(arguments) => write!(f, "{}({:?})", self.name, arguments),
            None => write!(f, "{}", self.name),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ModuleInitStatement {
    Use(UseStatement),
    Expression(Expression),
    Assignment(Assignment),
    FunctionDefinition(FunctionDefinition),
}

impl Parse for ModuleInitStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let first = pair.clone().into_inner().next().unwrap();
        with_pair_ok!(
            match first.as_rule() {
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
            },
            pair
        )
    }
}

#[derive(Clone, Debug)]
pub struct ModuleInitDefinition {
    parameters: ParameterList,
    body: Vec<ModuleInitStatement>,
}

impl Parse for ModuleInitDefinition {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::module_init_definition);
        let mut parameters = ParameterList::default();
        let mut body = Vec::new();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::parameter_list => {
                    parameters = ParameterList::parse(pair)?.value().clone();
                }
                Rule::module_init_statement => {
                    body.push(ModuleInitStatement::parse(pair)?.value().clone());
                }
                Rule::COMMENT => {}
                rule => unreachable!(
                    "expected parameter_list or module_init_statement. Instead found {rule:?}"
                ),
            }
        }

        with_pair_ok!(ModuleInitDefinition { parameters, body }, pair)
    }
}

#[derive(Clone, Debug)]
pub struct ModuleBody {
    pub statements: Vec<ModuleStatement>,
    pub symbols: SymbolTable,
    pub inits: Vec<std::rc::Rc<ModuleInitDefinition>>,
}

impl ModuleBody {
    fn new() -> Self {
        Self {
            statements: Vec::new(),
            symbols: SymbolTable::new(),
            inits: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: ModuleStatement) {
        self.statements.push(statement.clone());
        match statement {
            ModuleStatement::FunctionDefinition(function) => {
                self.symbols.add(Symbol::Function(function));
            }
            ModuleStatement::ModuleDefinition(module) => {
                self.symbols.add(Symbol::ModuleDefinition(module));
            }
            ModuleStatement::ModuleInitDefinition(init) => {
                self.inits.push(init.clone());
            }
            _ => {}
        }
    }

    pub fn get_symbols_by_name(&self, name: &Identifier) -> Vec<&Symbol> {
        self.symbols.get(name)
    }

    pub fn symbols(&self) -> &SymbolTable {
        &self.symbols
    }

    pub fn get_symbol(&self, name: &Identifier) -> Option<&Symbol> {
        self.symbols.get(name).first().cloned()
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.add(symbol);
    }
}

impl Parse for ModuleBody {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::module_body);
        let mut body = ModuleBody::new();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::module_statement => {
                    let statement = ModuleStatement::parse(pair.clone())?.value().clone();
                    body.add_statement(statement);
                }
                Rule::expression => {
                    let expression = Expression::parse(pair.clone())?.value().clone();
                    body.add_statement(ModuleStatement::Expression(expression));
                }
                _ => {}
            }
        }

        with_pair_ok!(body, pair)
    }
}

impl Eval for ModuleBody {
    type Output = Node;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
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

#[derive(Clone, Debug)]
pub struct ForStatement {
    loop_var: Assignment,
    body: ModuleBody,
}

impl Parse for ForStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        Parser::ensure_rule(&pair, Rule::module_for_statement);

        let mut pairs = pair.into_inner();

        let loop_var = Assignment::parse(pairs.next().unwrap())?;
        let body = ModuleBody::parse(pairs.next().unwrap())?;

        with_pair_ok!(
            ForStatement {
                loop_var: loop_var.value().clone(),
                body: body.value().clone(),
            },
            p
        )
    }
}

impl std::fmt::Display for ForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "for {} {}", self.loop_var, self.body)
    }
}

#[derive(Clone, Debug, strum::IntoStaticStr)]
pub enum ModuleStatement {
    Use(UseStatement),
    Expression(Expression),
    For(ForStatement),
    Assignment(Assignment),
    ModuleDefinition(std::rc::Rc<ModuleDefinition>),
    FunctionDefinition(std::rc::Rc<FunctionDefinition>),
    ModuleInitDefinition(std::rc::Rc<ModuleInitDefinition>),
}

impl Parse for ModuleStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::module_statement);
        let first = pair.clone().into_inner().next().unwrap();
        with_pair_ok!(
            match first.as_rule() {
                Rule::use_statement => {
                    ModuleStatement::Use(UseStatement::parse(first)?.value().clone())
                }
                Rule::expression => {
                    ModuleStatement::Expression(Expression::parse(first)?.value().clone())
                }
                Rule::assignment => {
                    ModuleStatement::Assignment(Assignment::parse(first)?.value().clone())
                }
                Rule::module_for_statement => {
                    ModuleStatement::For(ForStatement::parse(first)?.value().clone())
                }
                Rule::module_definition | Rule::namespace_definition =>
                    ModuleStatement::ModuleDefinition(std::rc::Rc::new(
                        ModuleDefinition::parse(first)?.value().clone(),
                    )),
                Rule::module_init_definition => ModuleStatement::ModuleInitDefinition(
                    std::rc::Rc::new(ModuleInitDefinition::parse(first)?.value().clone(),)
                ),
                Rule::function_definition => ModuleStatement::FunctionDefinition(std::rc::Rc::new(
                    FunctionDefinition::parse(first)?.value().clone(),
                )),
                rule => unreachable!(
                    "Unexpected module statement, got {:?} {:?}",
                    rule,
                    first.clone()
                ),
            },
            pair
        )
    }
}

impl Eval for ModuleStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        match self {
            ModuleStatement::Use(use_statement) => {
                use_statement.eval(context)?;
            }
            ModuleStatement::Expression(expr) => {
                expr.eval(context)?;
            }
            ModuleStatement::Assignment(assignment) => {
                assignment.eval(context)?;
            }
            ModuleStatement::FunctionDefinition(function_definition) => {
                context.add_symbol(Symbol::Function(function_definition.clone()));
            }
            ModuleStatement::ModuleDefinition(module_definition) => {
                context.add_symbol(Symbol::ModuleDefinition(module_definition.clone()));
            }
            statement => {
                let s: &'static str = statement.into();
                unimplemented!("ModuleStatement::{s}")
            }
        }

        Ok(())
    }
}

impl std::fmt::Display for ModuleStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ModuleStatement::Use(use_statement) => write!(f, "{use_statement}"),
            ModuleStatement::Expression(expression) => write!(f, "{expression}"),
            ModuleStatement::Assignment(assignment) => write!(f, "{assignment}"),
            ModuleStatement::For(for_statement) => write!(f, "{for_statement}"),
            ModuleStatement::ModuleDefinition(module_definition) => {
                write!(f, "{}", module_definition.name)
            }
            ModuleStatement::FunctionDefinition(function_definition) => {
                write!(f, "{}", function_definition.name)
            }
            ModuleStatement::ModuleInitDefinition(_) => write!(f, "module init"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModuleDefinition {
    pub attributes: Vec<Attribute>,
    pub name: Identifier,
    pub parameters: Option<ParameterList>,
    pub body: ModuleBody,
}

impl ModuleDefinition {
    pub fn namespace(name: Identifier) -> Self {
        ModuleDefinition {
            attributes: Vec::new(),
            name,
            parameters: None,
            body: ModuleBody::new(),
        }
    }

    pub fn add_function(&mut self, function: std::rc::Rc<FunctionDefinition>) {
        self.body.add_symbol(Symbol::Function(function.clone()));
    }

    pub fn add_module(&mut self, module: std::rc::Rc<ModuleDefinition>) {
        self.body
            .add_symbol(Symbol::ModuleDefinition(module.clone()));
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.body.add_symbol(symbol);
    }

    pub fn get_symbols_by_name(&self, name: &Identifier) -> Vec<&Symbol> {
        self.body.get_symbols_by_name(name)
    }

    pub fn symbols(&self) -> &SymbolTable {
        self.body.symbols()
    }
}

impl Parse for ModuleDefinition {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut attributes = Vec::new();
        let mut name = Identifier::default();
        let mut parameters = None;
        let mut body = ModuleBody::new();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::attribute_list => {
                    attributes.push(Attribute::parse(pair)?.value().clone());
                }
                Rule::identifier => {
                    name = Identifier::parse(pair)?.value().clone();
                }
                Rule::parameter_list => {
                    parameters = Some(ParameterList::parse(pair)?.value().clone());
                }
                Rule::module_body => {
                    body = ModuleBody::parse(pair.clone())?.value().clone();
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
            pair
        )
    }
}

pub type BuiltinModuleFunctor = dyn Fn(&ArgumentMap, &mut Context) -> Result<Node, Error>;

#[derive(Clone)]
pub struct BuiltinModule {
    pub name: Identifier,
    pub parameters: ParameterList,
    pub f: &'static BuiltinModuleFunctor,
}

impl std::fmt::Debug for BuiltinModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BUILTIN_MOD({})", &self.name)
    }
}

impl BuiltinModule {
    pub fn new(
        name: Identifier,
        parameters: ParameterList,
        f: &'static BuiltinModuleFunctor,
    ) -> Self {
        Self {
            name,
            parameters,
            f,
        }
    }

    pub fn call(&self, args: &CallArgumentList, context: &mut Context) -> Result<Node, Error> {
        let arg_map = args
            .eval(context)?
            .get_matching_arguments(&self.parameters.eval(context)?)?;
        (self.f)(&arg_map, context)
    }
}

#[macro_export]
macro_rules! builtin_module {
    // This macro is used to create a BuiltinModule from a function
    ($name:ident, $f:expr) => {
        BuiltinModule::new(
            stringify!($name).into(),
            &$f,
        )
    };
    // This macro is used to create a BuiltinModule from a function with no arguments
    ($name:ident) => {
        BuiltinModule::new(
            stringify!($name).into(),
            microcad_parser::language::parameter::ParameterList::default(),
            &|_, ctx| Ok(ctx.append_node($name())),
        )
    };
    ($name:ident($($arg:ident: $type:ident),*)) => {
        BuiltinModule::new(
            stringify!($name).into(),
            microcad_parser::parameter_list![$(microcad_parser::parameter!($arg: $type)),*],
            &|args, ctx| {
                let mut l = |$($arg: $type),*| Ok(ctx.append_node($name($($arg),*)));
                let ($($arg),*) = (
                    $(args.get(&stringify!($arg).into()).unwrap().clone().try_into()?),*
                );
                l($($arg),*)
            },
        )
    };
}

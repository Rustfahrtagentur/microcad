use crate::{eval::*, language::*, parser::*, with_pair_ok};
use microcad_render::tree;

#[derive(Clone, Debug, Default)]
pub struct ModuleBody {
    pub statements: Vec<ModuleStatement>,
    pub symbols: SymbolTable,
    pub inits: Vec<std::rc::Rc<ModuleInitDefinition>>,
}

impl ModuleBody {
    pub fn new() -> Self {
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

impl Symbols for ModuleBody {
    fn find_symbols(&self, name: &Identifier) -> Vec<&Symbol> {
        self.symbols.find_symbols(name)
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
    type Output = tree::Node;

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

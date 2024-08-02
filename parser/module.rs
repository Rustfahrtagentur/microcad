// Resolve a qualified name to a type or value.

use crate::call::Call;
use crate::identifier::{Identifier, QualifiedName};
use crate::parser::*;
use pest::pratt_parser::PrattParser;

#[derive(Default)]
pub struct ModuleNested(Vec<Call>);

impl Parse for ModuleNested {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        Ok(ModuleNested(Parser::vec(pair.into_inner(), Call::parse)?))
    }
}
/*impl Parse for ModuleNodeExpression {
    fn parse(pair: Pair) -> Result<Self, ParseError> {


    }
}*/

/*

pub struct _Module {
    name: Identifier,
    constructor: Vec<FunctionArgument>,
}

trait ParseNode {
    fn parse_node(pair: Pair, root: SyntaxNode) -> Result<Self, ParseError>;
}

trait Build {
    fn build(
        self,
        node: SyntaxNode,
        context: &mut Context,
    ) -> Result<crate::tree::Node, ParseError>;
}

fn build(root: SyntaxNode) -> Result<crate::tree::Node, BuildError> {
    let mut context = Context::default();
    let mut tree = Tree::default();

    for node in root.children() {
        tree.append_node(node);
    }

    Ok(node)
}

*/

pub struct UseAlias(pub QualifiedName, pub Identifier);

impl std::fmt::Display for UseAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "use {:?} as {:?}", self.0, self.1)
    }
}

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
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();
        Ok(UseAlias(
            QualifiedName::parse(pairs.next().unwrap())?,
            Identifier::parse(pairs.next().unwrap())?,
        ))
    }
}

impl Parse for UseStatement {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();

        let first = pairs.next().unwrap();
        let second = pairs.next();

        match first.as_rule() {
            Rule::qualified_name_list => {
                let qualified_name_list = Parser::vec(first.into_inner(), QualifiedName::parse)?;
                if let Some(second) = second {
                    if second.as_rule() == Rule::qualified_name {
                        return Ok(UseStatement::UseFrom(
                            qualified_name_list,
                            QualifiedName::parse(second)?,
                        ));
                    } else {
                        unreachable!();
                    }
                } else {
                    return Ok(UseStatement::Use(qualified_name_list));
                }
            }
            Rule::qualified_name_all => {
                if let Some(second) = second {
                    if second.as_rule() == Rule::qualified_name_list {
                        return Ok(UseStatement::UseAll(Parser::vec(
                            second.into_inner(),
                            QualifiedName::parse,
                        )?));
                    } else {
                        unreachable!();
                    }
                }
            }
            Rule::use_alias => {
                return Ok(UseStatement::UseAlias(UseAlias::parse(first)?));
            }

            _ => unreachable!(),
        }

        Err(ParseError::InvalidUseStatement)
    }
}

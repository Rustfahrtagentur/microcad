// Resolve a qualified name to a type or value.

use crate::call::Call;
use crate::identifier::{Identifier, QualifiedName};
use crate::parser::*;
use pest::pratt_parser::PrattParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(module_node_union, Left) | Op::infix(module_node_difference, Left))
            .op(Op::infix(module_node_intersection, Left) | Op::infix(module_node_xor, Left))
    };
}

#[derive(Default)]
pub struct ModuleNested(Vec<Call>);

impl Parse for ModuleNested {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        Ok(ModuleNested(Parser::vec(pair.into_inner(), Call::parse)?))
    }
}

#[derive(Default)]
pub enum ModuleNodeExpression {
    /// Something went wrong (and an error will be reported)
    #[default]
    Invalid,

    Identifier(Identifier),

    Nested(ModuleNested),

    /// A binary operation: a | b
    BinaryOp {
        lhs: Box<ModuleNodeExpression>,
        /// '|', '-', '&', '^'
        op: char,
        rhs: Box<ModuleNodeExpression>,
    },

    /// A unary operation: !a
    UnaryOp {
        /// '!'
        op: char,
        rhs: Box<ModuleNodeExpression>,
    },
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

    /// Import as alias from a module: `use a as b from c`
    UseAliasFrom(UseAlias, QualifiedName),
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
            UseStatement::UseAliasFrom(alias, from) => write!(f, "{} from {}", alias, from),
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
                if let Some(second) = second {
                    return Ok(UseStatement::UseAliasFrom(
                        UseAlias::parse(first)?,
                        QualifiedName::parse(second)?,
                    ));
                } else {
                    return Ok(UseStatement::UseAlias(UseAlias::parse(first)?));
                }
            }

            _ => unreachable!(),
        }

        Err(ParseError::InvalidUseStatement)
    }
}

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;

use crate::expression::Expression;
use crate::identifier::{Identifier, IdentifierListError, QualifiedName};
use thiserror::Error;

pub type Pair<'i> = pest::iterators::Pair<'i, Rule>;
pub type Pairs<'i> = pest::iterators::Pairs<'i, Rule>;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Expected identifier")]
    ExpectedIdentifier,
    #[error("Invalid use statement")]
    InvalidUseStatement,
    #[error("Error parsing floating point literal: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Error parsing integer literal: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Error parsing color literal: {0}")]
    ParseColorError(String),
    #[error("Unknown unit: {0}")]
    UnknownUnit(String),
    #[error("Unexpected token")]
    UnexpectedToken,
    #[error("Type error: {0}")]
    TypeError(#[from] crate::lang_type::TypeError),
    #[error("Identifier list error: {0}")]
    IdentifierListError(#[from] IdentifierListError),
    #[error("Tuple expression contains both named and positional arguments")]
    MixedTupleArguments,
    #[error("Duplicate named argument: {0}")]
    DuplicateNamedArgument(Identifier),
    #[error("Positional argument after named argument")]
    PositionalArgumentAfterNamed,
    #[error("Empty tuple expression")]
    EmptyTupleExpression,
}

pub trait Parse: Sized {
    fn parse(pair: Pair) -> Result<Self, ParseError>;
}

pub enum FunctionArgument {
    PositionalArgument(Expression),
    NamedArgument(Identifier, Expression),
}

impl FunctionArgument {
    pub fn name(&self) -> Option<&Identifier> {
        match self {
            Self::PositionalArgument(_) => None,
            Self::NamedArgument(ident, _) => Some(ident),
        }
    }

    pub fn expression(&self) -> &Expression {
        match self {
            Self::PositionalArgument(expr) => expr,
            Self::NamedArgument(_, expr) => expr,
        }
    }
}

#[derive(Default)]
pub struct FunctionCall {
    pub qualified_name: QualifiedName,
    pub function_argument_list: Vec<FunctionArgument>,
}

pub struct ModuleNodeStatement {
    pub ident: Option<Identifier>,
    pub calls: Vec<FunctionCall>,
    pub has_inner: bool,
}

impl Parser {
    /// @brief Helper function to parse a vector of pairs into a vector of T
    /// @param pairs The pairs to parse
    /// @param f The function to parse the pair into T
    /// @return A vector of T
    pub fn vec<T>(
        pairs: Pairs,
        f: impl Fn(Pair) -> Result<T, ParseError>,
    ) -> Result<Vec<T>, ParseError> {
        Ok(pairs.map(f).map(|x| x.unwrap()).collect::<Vec<_>>())
    }

    /// Convenience function to parse a rule for type `T` and panic on error
    pub fn parse_rule_or_panic<T>(rule: Rule, input: &str) -> T
    where
        T: Parse,
    {
        use pest::Parser;
        let pair = crate::parser::Parser::parse(rule, input)
            .unwrap()
            .next()
            .unwrap();
        T::parse(pair).unwrap()
    }

    fn function_argument(pair: Pair) -> Result<FunctionArgument, ParseError> {
        let pairs = pair.into_inner();
        let first = pairs.clone().nth(0).unwrap();
        let second = pairs.clone().nth(1).unwrap();

        match first.as_rule() {
            Rule::identifier => Ok(FunctionArgument::NamedArgument(
                Identifier::parse(first)?,
                Expression::parse(second)?,
            )),
            Rule::expression => Ok(FunctionArgument::PositionalArgument(Expression::parse(
                first,
            )?)),
            _ => unreachable!(),
        }
    }

    fn function_argument_list(pairs: Pairs) -> Result<Vec<FunctionArgument>, ParseError> {
        Self::vec(pairs, Self::function_argument)
    }

    fn function_call(pairs: Pairs) -> Result<FunctionCall, ParseError> {
        let mut call = FunctionCall::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::qualified_name => {
                    call.qualified_name = QualifiedName::parse(pair)?;
                }
                Rule::call_argument_list => {
                    call.function_argument_list = Self::function_argument_list(pair.into_inner())?;
                }
                _ => unreachable!(),
            }
        }

        Ok(call)
    }

    pub fn module_node_statement(pairs: Pairs) -> Result<ModuleNodeStatement, ParseError> {
        let mut module_node_statement = ModuleNodeStatement {
            ident: Default::default(),
            calls: Vec::new(),
            has_inner: false,
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::module_node_inner => {
                    module_node_statement.has_inner = true;
                }
                Rule::call => {
                    let call = Self::function_call(pair.into_inner())?;
                    module_node_statement.calls.push(call);
                }
                _ => {
                    unreachable!(
                        "Expr::parse expected call or module_node_inner, found {:?}",
                        pair.as_rule()
                    );
                }
            }
        }

        if module_node_statement.calls.is_empty() {
            panic!("No calls found in module node statement");
        } else {
            Ok(module_node_statement)
        }
    }
}

#[cfg(test)]
mod tests {
    include!(concat!(env!("OUT_DIR"), "/pest_test.rs"));
    use literal::NumberLiteral;
    use parser::Parse;

    #[test]
    fn number_literal() {
        use pest::Parser;
        let pairs = crate::parser::Parser::parse(parser::Rule::number_literal, "90.0°");

        assert!(pairs.is_ok());
        let pair = pairs.unwrap().next().unwrap();

        let literal::NumberLiteral(number, unit) = NumberLiteral::parse(pair).unwrap();

        assert_eq!(number, 90.0);
        assert_eq!(unit, units::Unit::DegS);
    }

    fn _test_file(path: impl AsRef<std::path::Path>) {
        use pest::Parser;

        let input = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Test file not found: {:?}", path.as_ref()));
        assert!(crate::parser::Parser::parse(parser::Rule::document, &input).is_ok())
    }

    //#[test]
    fn _test_file_nested() {
        _test_file("tests/nested.csg");
        _test_file("tests/module.csg");
    }
}

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;

use crate::language::{identifier::*, lang_type::*};
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
    TypeError(#[from] TypeError),
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
    #[error("Missing type or value for definition parameter: {0}")]
    DefinitionParameterMissingTypeOrValue(Identifier),
}

pub struct WithPair<'a, T> {
    value: T,
    pair: Pair<'a>,
}

#[macro_export]
macro_rules! with_pair_ok {
    ($value:expr, $pair:ident) => {
        Ok($crate::parser::WithPair::new($value, $pair))
    };
    () => {};
}

impl<'a, T> WithPair<'a, T> {
    pub fn new(value: T, pair: Pair<'a>) -> Self {
        Self { value, pair }
    }

    pub fn pair(&self) -> Pair<'a> {
        self.pair.clone()
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn rule(&self) -> Rule {
        self.pair.as_rule()
    }

    pub fn start_pos(&self) -> pest::Position<'a> {
        self.pair.as_span().start_pos()
    }

    pub fn end_pos(&self) -> pest::Position<'a> {
        self.pair.as_span().end_pos()
    }
}

impl<'a, T> std::ops::Deref for WithPair<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub type ParseResult<'a, T> = Result<WithPair<'a, T>, ParseError>;

pub trait Parse: Sized {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self>;
}

impl Parser {
    /// @brief Helper function to parse a vector of pairs into a vector of T
    /// @param pairs The pairs to parse
    /// @param f The function to parse the pair into T
    /// @return A vector of T
    pub fn vec<'a, T>(
        pair: Pair<'a>,
        f: impl Fn(Pair<'a>) -> ParseResult<'a, T>,
    ) -> ParseResult<'a, Vec<T>>
    where
        T: Clone,
    {
        let mut vec = Vec::new();
        for pair in pair.clone().into_inner() {
            vec.push(f(pair)?.value().clone());
        }

        with_pair_ok!(vec, pair)
    }

    /// Convenience function to parse a rule for type `T` and panic on error
    pub fn parse_rule_or_panic<T>(rule: Rule, input: &str) -> T
    where
        T: Parse + Clone,
    {
        use pest::Parser;
        let pair = crate::parser::Parser::parse(rule, input.trim())
            .unwrap()
            .next()
            .unwrap();

        let w = T::parse(pair).unwrap();
        w.value().clone()
    }

    pub fn ensure_rule(pair: &Pair, expected: Rule) {
        let rule = pair.as_rule();
        assert_eq!(rule, expected, "Unexpected rule: {:?}", rule);
    }
}

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/pest_test.rs"));
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/md_test.rs"));
/*use literal::NumberLiteral;
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
}*/

//! µCAD Code Parser

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;

use crate::{errors::*, parse::*};
use thiserror::Error;

pub type Pair<'i> = pest::iterators::Pair<'i, Rule>;
pub type Pairs<'i> = pest::iterators::Pairs<'i, Rule>;

/// Parsing errors
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
    ParameterMissingTypeOrValue(Identifier),

    #[error("Duplicate parameter: {0}")]
    DuplicateParameter(Identifier),

    #[error("Duplicate argument: {0}")]
    DuplicateCallArgument(Identifier),
}

pub type ParseResult<T> = Result<T, ParseError>;

pub trait Parse: Sized {
    fn parse(pair: Pair<'_>) -> ParseResult<Self>;
}

impl Parser {
    /// Helper function to parse a vector of pairs into a vector of T
    ///
    /// # Arguments
    ///
    /// - `pairs`: The pairs to parse
    /// - `f`: The function to parse the pair into `T`
    ///
    /// Returns a vector of `T`
    pub fn vec<'a, T>(pair: Pair<'a>, f: impl Fn(Pair<'a>) -> ParseResult<T>) -> ParseResult<Vec<T>>
    where
        T: Clone,
    {
        let mut vec = Vec::new();
        for pair in pair.clone().into_inner() {
            vec.push(f(pair)?);
        }

        Ok(vec)
    }

    /// Parse a rule for type `T`
    pub fn parse_rule<T>(rule: Rule, input: &str) -> anyhow::Result<T>
    where
        T: Parse + Clone,
    {
        use pest::Parser as _;

        if let Some(pair) = Parser::parse(rule, input.trim())?.next() {
            Ok(T::parse(pair)?)
        } else {
            Err(anyhow::Error::msg("could not parse"))
        }
    }

    /// Convenience function to parse a rule for type `T` and panic on error
    pub fn parse_rule_or_panic<T>(rule: Rule, input: &str) -> T
    where
        T: Parse + Clone,
    {
        use pest::Parser as _;

        let no_match = format!("Rule {rule:?} does not match");
        let pair = Parser::parse(rule, input.trim())
            .expect(&no_match)
            .next()
            .unwrap();
        T::parse(pair).unwrap()
    }

    pub fn ensure_rule(pair: &Pair, expected: Rule) {
        let rule = pair.as_rule();
        assert_eq!(rule, expected, "Unexpected rule: {rule:?}");
    }
}

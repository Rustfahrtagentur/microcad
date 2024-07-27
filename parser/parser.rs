use std::str::FromStr;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;

use crate::expression::Expression;
use crate::literal::NumberLiteral;
use crate::units::Unit;

pub type Pair<'i> = pest::iterators::Pair<'i, Rule>;
pub type Pairs<'i> = pest::iterators::Pairs<'i, Rule>;

#[derive(Debug)]
pub enum ParseError {
    ExpectedIdentifier,
    ObjectNodeAtLeastOneCall,
    InvalidUseStatement,
    ParseFloatError(std::num::ParseFloatError),
    UnknownUnit(String),
}

impl From<std::num::ParseFloatError> for ParseError {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
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

pub struct ObjectNodeStatement {
    pub ident: Option<Identifier>,
    pub calls: Vec<FunctionCall>,
    pub has_inner: bool,
}

pub enum Visibility {
    Private,
    Public,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Identifier(String);

impl Identifier {
    /// @brief Every identifier starting with '_' is private
    pub fn visibility(self) -> Visibility {
        if self.0.starts_with('_') {
            Visibility::Private
        } else {
            Visibility::Public
        }
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, Clone)]
pub struct QualifiedName(Vec<Identifier>);

impl std::fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .0
            .iter()
            .map(|ident| ident.0.clone())
            .collect::<Vec<_>>()
            .join(".");
        write!(f, "{}", s)
    }
}

impl From<&str> for QualifiedName {
    fn from(value: &str) -> Self {
        let mut name = Vec::new();
        for ident in value.split('.') {
            name.push(Identifier(ident.to_string()));
        }
        Self(name)
    }
}

impl From<QualifiedName> for String {
    fn from(value: QualifiedName) -> Self {
        let s = value
            .0
            .iter()
            .map(|ident| ident.0.clone())
            .collect::<Vec<_>>()
            .join(".");
        s
    }
}

impl Parser {
    /// @brief Helper function to parse a vector of pairs into a vector of T
    /// @param pairs The pairs to parse
    /// @param rule The rule to match
    /// @param f The function to parse the pair into T
    /// @return A vector of T
    fn list<T>(
        pairs: Pairs,
        rule: Rule,
        f: impl Fn(Pair) -> Result<T, ParseError>,
    ) -> Result<Vec<T>, ParseError> {
        let mut vec = Vec::new();
        for pair in pairs {
            if pair.as_rule() == rule {
                vec.push(f(pair).unwrap());
            }
        }
        Ok(vec)
    }

    pub fn number_literal(pair: Pair) -> Result<NumberLiteral, ParseError> {
        assert_eq!(pair.as_rule(), Rule::number_literal);

        let mut pairs = pair.into_inner();
        let number_token = pairs.next().unwrap();

        assert_eq!(number_token.as_rule(), Rule::number);

        let value = number_token.as_str().parse::<f64>()?;
        let mut unit = Unit::None;

        if let Some(unit_token) = pairs.next() {
            unit = Unit::from_str(unit_token.as_str())
                .map_err(|u| ParseError::UnknownUnit(unit_token.to_string()))?;
        }
        Ok(NumberLiteral(value, unit))
    }

    fn expression(pair: Pair) -> Result<Expression, ParseError> {
        Expression::parse(pair)
    }

    fn function_argument(pair: Pair) -> Result<FunctionArgument, ParseError> {
        let pairs = pair.into_inner();
        let first = pairs.clone().nth(0).unwrap();
        let second = pairs.clone().nth(1).unwrap();

        match first.as_rule() {
            Rule::identifier => Ok(FunctionArgument::NamedArgument(
                Self::identifier(first)?,
                Self::expression(second)?,
            )),
            Rule::expression => Ok(FunctionArgument::PositionalArgument(Self::expression(
                first,
            )?)),
            _ => unreachable!(),
        }
    }

    fn function_argument_list(pairs: Pairs) -> Result<Vec<FunctionArgument>, ParseError> {
        Self::list(pairs, Rule::call_named_argument, Self::function_argument)
    }

    pub fn identifier(pair: Pair) -> Result<Identifier, ParseError> {
        if pair.as_rule() == Rule::identifier {
            Ok(Identifier(pair.as_span().as_str().into()))
        } else {
            Err(ParseError::ExpectedIdentifier)
        }
    }

    pub fn qualified_name(pair: Pair) -> Result<QualifiedName, ParseError> {
        let pairs = pair.into_inner();
        Ok(QualifiedName(Self::list::<Identifier>(
            pairs,
            Rule::identifier,
            Self::identifier,
        )?))
    }

    pub fn qualified_name_list(pairs: Pairs) -> Result<Vec<QualifiedName>, ParseError> {
        Self::list(pairs, Rule::qualified_name, Self::qualified_name)
    }

    fn function_call(pairs: Pairs) -> Result<FunctionCall, ParseError> {
        let mut call = FunctionCall::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::qualified_name => {
                    call.qualified_name = Self::qualified_name(pair)?;
                }
                Rule::call_argument_list => {
                    call.function_argument_list = Self::function_argument_list(pair.into_inner())?;
                }
                _ => unreachable!(),
            }
        }

        Ok(call)
    }

    fn object_node_id_assignment(pairs: Pairs) -> Result<Identifier, ParseError> {
        if let Some(pair) = pairs.peek() {
            Self::identifier(pair)
        } else {
            Err(ParseError::ExpectedIdentifier)
        }
    }

    pub fn object_node_statement(pairs: Pairs) -> Result<ObjectNodeStatement, ParseError> {
        let mut object_node_statement = ObjectNodeStatement {
            ident: Default::default(),
            calls: Vec::new(),
            has_inner: false,
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::object_node_inner => {
                    object_node_statement.has_inner = true;
                }
                Rule::call => {
                    let call = Self::function_call(pair.into_inner())?;
                    object_node_statement.calls.push(call);
                }
                _ => {
                    unreachable!(
                        "Expr::parse expected call or object_node_inner, found {:?}",
                        pair.as_rule()
                    );
                }
            }
        }

        if object_node_statement.calls.is_empty() {
            Err(ParseError::ObjectNodeAtLeastOneCall)
        } else {
            Ok(object_node_statement)
        }
    }
}

#[cfg(test)]
mod tests {
    include!(concat!(env!("OUT_DIR"), "/pest_test.rs"));
    use crate::parser::Parser;

    #[test]
    fn number_literal() {
        use pest::Parser;
        let pairs = crate::parser::Parser::parse(parser::Rule::number_literal, "90°");

        assert!(pairs.is_ok());
        let pair = pairs.unwrap().next().unwrap();

        let literal::NumberLiteral(number, unit) =
            crate::parser::Parser::number_literal(pair).unwrap();

        assert_eq!(number, 90.0);
        assert_eq!(unit, units::Unit::DegS);
    }

    #[test]
    fn object_node_statement() {
        use pest::Parser;
        let pairs = crate::parser::Parser::parse(
            parser::Rule::object_node_assignment,
            "node_id := translate(x = 5.0mm) rotate(angle = 90°) { rectangle(width = 5.0mm); }",
        );

        assert!(pairs.is_ok());
        let mut pairs = pairs.unwrap();
        let pair = pairs.next().unwrap();

        let object_node_statement =
            crate::parser::Parser::object_node_statement(pair.into_inner()).unwrap();

        assert_eq!(object_node_statement.calls.len(), 2);
        assert_eq!(object_node_statement.ident, Some("node_id".into()));
        assert!(object_node_statement.has_inner);

        // Test function call
        {
            let call = object_node_statement.calls.first().unwrap();
            assert_eq!(call.qualified_name.to_string(), "translate".to_string());
            assert_eq!(call.function_argument_list.len(), 1);
        }
    }

    fn test_file(path: impl AsRef<std::path::Path>) {
        use pest::Parser;

        let input = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Test file not found: {:?}", path.as_ref()));
        assert!(crate::parser::Parser::parse(parser::Rule::document, &input).is_ok())
    }

    #[test]
    fn test_file_nested() {
        test_file("tests/nested.csg");
        test_file("tests/module.csg");
    }
}

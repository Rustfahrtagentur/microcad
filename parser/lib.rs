use pest::iterators::{Pair, Pairs};
#[allow(unused_imports)]
use pest::Parser;
use pest_derive::Parser;

mod diagnostics;
pub mod syntaxtree;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CsglParser;

#[derive(Default, Clone)]
struct Expression {
    literal: String,
}

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        Self {
            literal: value.to_string(),
        }
    }
}

#[derive(Default, Clone)]
struct FunctionArgument {
    ident: Identifier,
    expression: Expression,
}

#[derive(Default, Clone)]
struct FunctionCall {
    qualified_name: QualifiedName,
    function_argument_list: Vec<FunctionArgument>,
}

struct ObjectNodeStatement {
    ident: Option<Identifier>,
    calls: Vec<FunctionCall>,
    has_inner: bool,
}

enum Visibility {
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

impl CsglParser {
    fn expression(pairs: Pairs<Rule>) -> Result<Expression, ()> {
        Ok(Expression {
            literal: pairs.clone().next().unwrap().into_inner().to_string(),
        })
    }

    fn function_argument(pairs: Pairs<Rule>) -> Result<FunctionArgument, ()> {
        Ok(FunctionArgument {
            ident: Self::identifier(pairs.clone().nth(0).unwrap())?,
            expression: Self::expression(pairs).unwrap(),
        })
    }

    fn function_argument_list(pairs: Pairs<Rule>) -> Result<Vec<FunctionArgument>, ()> {
        let mut args = Vec::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::function_argument => {
                    args.push(Self::function_argument(pair.into_inner()).unwrap());
                }
                _ => unreachable!(),
            }
        }
        Ok(args)
    }

    fn identifier(pair: Pair<Rule>) -> Result<Identifier, ()> {
        if pair.as_rule() == Rule::ident {
            Ok(Identifier(pair.as_span().as_str().into()))
        } else {
            Err(())
        }
    }

    fn qualified_name(pairs: Pairs<Rule>) -> Result<QualifiedName, ()> {
        let mut name = Vec::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::ident => {
                    name.push(Self::identifier(pair)?);
                }
                _ => unreachable!(),
            }
        }
        Ok(QualifiedName(name))
    }

    fn function_call(pairs: Pairs<Rule>) -> Result<FunctionCall, ()> {
        let mut call = FunctionCall::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::qualified_name => {
                    call.qualified_name = Self::qualified_name(pair.into_inner())?;
                }
                Rule::function_argument_list => {
                    call.function_argument_list = Self::function_argument_list(pair.into_inner())?;
                }
                _ => unreachable!(),
            }
        }

        Ok(call)
    }

    fn object_node_id_assignment(pairs: Pairs<Rule>) -> Result<Identifier, ()> {
        if let Some(pair) = pairs.peek() {
            Self::identifier(pair)
        } else {
            Err(())
        }
    }

    fn object_node_statement(pairs: Pairs<Rule>) -> Result<ObjectNodeStatement, ()> {
        let mut object_node_statement = ObjectNodeStatement {
            ident: Default::default(),
            calls: Vec::new(),
            has_inner: false,
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::object_node_id_assignment => {
                    object_node_statement.ident =
                        Some(Self::object_node_id_assignment(pair.into_inner())?);
                }
                Rule::object_node_inner => {
                    object_node_statement.has_inner = true;
                }
                Rule::function_call => {
                    let call = Self::function_call(pair.into_inner())?;
                    object_node_statement.calls.push(call);
                }
                _ => {
                    println!("{:?}", pair.as_rule());
                    unreachable!();
                }
            }
        }

        if object_node_statement.calls.is_empty() {
            Err(())
        } else {
            Ok(object_node_statement)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    include!(concat!(env!("OUT_DIR"), "/pest_test.rs"));

    #[test]
    fn object_node_statement() {
        let pairs = CsglParser::parse(
            Rule::object_node_statement,
            "node_id := translate(x = 5.0mm) rotate(angle = 90°) { rectangle(width = 5.0mm); }",
        );

        assert!(pairs.is_ok());
        let mut pairs = pairs.unwrap();
        let pair = pairs.next().unwrap();

        let object_node_statement = CsglParser::object_node_statement(pair.into_inner()).unwrap();

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

    #[test]
    fn test_file_nested() {
        let test_filename = "tests/nested.csg";
        let input = std::fs::read_to_string(test_filename)
            .unwrap_or_else(|_| panic!("Test file not found: {}", test_filename));

        assert!(CsglParser::parse(Rule::document, &input).is_ok())
    }
}

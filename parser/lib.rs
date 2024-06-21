use pest::iterators::{Pair, Pairs};
#[allow(unused_imports)]
use pest::Parser;
use pest_derive::Parser;
use syntaxtree::UseStatement;

mod diagnostics;
mod module;

pub mod syntaxtree;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CsglParser;

pub trait Parse {
    fn parse(pair: Pair<Rule>) -> Self;
}

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
    /// @brief Helper function to parse a vector of pairs into a vector of T
    /// @param pairs The pairs to parse
    /// @param rule The rule to match
    /// @param f The function to parse the pair into T
    /// @return A vector of T
    fn list<T>(
        pairs: Pairs<Rule>,
        rule: Rule,
        f: impl Fn(Pair<Rule>) -> Result<T, ()>,
    ) -> Result<Vec<T>, ()> {
        let mut vec = Vec::new();
        for pair in pairs {
            if pair.as_rule() == rule {
                vec.push(f(pair).unwrap());
            }
        }
        Ok(vec)
    }

    fn expression(pairs: Pairs<Rule>) -> Result<Expression, ()> {
        Ok(Expression {
            literal: pairs.clone().next().unwrap().into_inner().to_string(),
        })
    }

    fn function_argument(pair: Pair<Rule>) -> Result<FunctionArgument, ()> {
        let pairs = pair.into_inner();
        Ok(FunctionArgument {
            ident: Self::identifier(pairs.clone().nth(0).unwrap())?,
            expression: Self::expression(pairs).unwrap(),
        })
    }

    fn function_argument_list(pairs: Pairs<Rule>) -> Result<Vec<FunctionArgument>, ()> {
        Self::list(pairs, Rule::function_argument, Self::function_argument)
    }

    fn identifier(pair: Pair<Rule>) -> Result<Identifier, ()> {
        if pair.as_rule() == Rule::ident {
            Ok(Identifier(pair.as_span().as_str().into()))
        } else {
            Err(())
        }
    }

    fn qualified_name(pair: Pair<Rule>) -> Result<QualifiedName, ()> {
        let pairs = pair.into_inner();
        Ok(QualifiedName(Self::list::<Identifier>(
            pairs,
            Rule::ident,
            Self::identifier,
        )?))
    }

    fn qualified_name_list(pairs: Pairs<Rule>) -> Result<Vec<QualifiedName>, ()> {
        Self::list(pairs, Rule::qualified_name, Self::qualified_name)
    }

    fn function_call(pairs: Pairs<Rule>) -> Result<FunctionCall, ()> {
        let mut call = FunctionCall::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::qualified_name => {
                    call.qualified_name = Self::qualified_name(pair)?;
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

    fn use_statement(pairs: Pairs<Rule>) -> Result<UseStatement, ()> {
        let mut use_statement = UseStatement {
            qualified_names: Default::default(),
            from: Vec::new(),
            alias: None,
        };

        // @todo: Implement use statement parsing

        Ok(use_statement)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    include!(concat!(env!("OUT_DIR"), "/pest_test.rs"));

    #[test]
    fn object_node_statement() {
        use pest::Parser;
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
        use pest::Parser;
        assert!(CsglParser::parse(Rule::document, &input).is_ok())
    }
}

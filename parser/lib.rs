use pest::iterators::{Pair, Pairs};
#[allow(unused_imports)]
use pest::Parser;
use pest_derive::Parser;

mod diagnostics;
//mod document;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct CsglParser;

#[derive(Default, Clone)]
struct Expression {
    literal: String,
}

#[derive(Default, Clone)]
struct FunctionArgument {
    ident: Identifier,
    expression: Expression,
}

#[derive(Default, Clone)]
struct FunctionCall {
    ident: Identifier,
    function_argument_list: Vec<FunctionArgument>,
}

struct ObjectNodeStatement {
    id: Option<Identifier>,
    calls: Vec<FunctionCall>,
    has_inner: bool,
}

enum Visibility {
    Private,
    Public,
}

#[derive(Default, Clone, PartialEq)]
struct Identifier(String);

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

struct QualifiedName(Vec<Identifier>);

impl CsglParser {
    /*
        fn code(pairs: Pairs<Rule>) {
            for pair in pairs {
                match pair.as_rule() {
                    Rule::object_node_statement => {
                        let pairs = pair.into_inner();
                        println!("node statement: {pairs:#?}");
                        Self::node_statement(pairs).unwrap();
                    }
                    _ => Self::code(pair.into_inner()),
                }
            }
        }
    */
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
            Ok(Identifier(pair.to_string()))
        } else {
            Err(())
        }
    }

    fn function_call(pairs: Pairs<Rule>) -> Result<FunctionCall, ()> {
        let mut call = FunctionCall::default();

        for pair in pairs {
            match pair.as_rule() {
                Rule::ident => {
                    call.ident = Self::identifier(pair)?;
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
            id: Default::default(),
            calls: Vec::new(),
            has_inner: false,
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::object_node_id_assignment => {
                    object_node_statement.id =
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
    fn test_file_nested() {
        let test_filename = "tests/nested.csg";
        let input = std::fs::read_to_string(test_filename)
            .unwrap_or_else(|_| panic!("Test file not found: {}", test_filename));

        //use log::trace;
        //assert!(!input.is_empty());
        //trace!("{input}");
        println!("{input}");

        match CsglParser::parse(Rule::r#code, &input) {
            Ok(pairs) => {
                //CsglParser::code(pairs);
                //println!("{pairs:#?}");
            }
            Err(e) => {
                panic!("Failed parsing file in {}: {}", test_filename, e);
            }
        }
    }
}

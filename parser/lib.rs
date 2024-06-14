use pest::iterators::Pairs;
#[allow(unused_imports)]
use pest::Parser;

use pest_derive::Parser;

mod moduletree;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CsglParser;

trait Statement {
    fn run() {}
}

struct Expression {
    literal: String,
}

struct FunctionArgument {
    ident: String,
    expression: Expression,
}

struct FunctionCall {
    ident: String,
    function_argument_list: Vec<FunctionArgument>,
}

struct NodeStatement {
    ident: String,
    function_argument_list: Vec<FunctionArgument>,
}

impl Statement for NodeStatement {}

impl CsglParser {
    fn code(pairs: Pairs<Rule>) {
        for pair in pairs {
            match pair.as_rule() {
                Rule::node_statement => {
                    let pairs = pair.into_inner();
                    println!("node statement: {pairs:#?}");
                    Self::node_statement(pairs).unwrap();
                }
                _ => Self::code(pair.into_inner()),
            }
        }
    }

    fn expression(pairs: Pairs<Rule>) -> Result<Expression, ()> {
        Ok(Expression {
            literal: pairs.clone().next().unwrap().into_inner().to_string(),
        })
    }

    fn function_argument(pairs: Pairs<Rule>) -> Result<FunctionArgument, ()> {
        Ok(FunctionArgument {
            ident: pairs.clone().next().unwrap().to_string(),
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

    fn node_statement(pairs: Pairs<Rule>) -> Result<NodeStatement, ()> {
        let mut node_statement = NodeStatement {
            ident: Default::default(),
            function_argument_list: Vec::default(),
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::function_call => return Self::node_statement(pair.into_inner()),
                Rule::ident => node_statement.ident = pair.to_string(),
                Rule::function_argument_list => {
                    node_statement.function_argument_list =
                        Self::function_argument_list(pair.into_inner()).unwrap()
                }
                Rule::node_name_assignment => {}
                Rule::node_inner => return Self::node_statement(pair.into_inner()),
                _ => {
                    println!("{:?}", pair.as_rule());
                    unreachable!();
                }
            }
        }

        Ok(node_statement)
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
            .expect(format!("Test file not found: {}", test_filename).as_str());

        //use log::trace;
        //assert!(!input.is_empty());
        //trace!("{input}");
        println!("{input}");

        match CsglParser::parse(Rule::r#code, &input) {
            Ok(pairs) => {
                CsglParser::code(pairs);
                //println!("{pairs:#?}");
            }
            Err(e) => {
                panic!("Failed parsing file in {}: {}", test_filename, e);
            }
        }
    }
}

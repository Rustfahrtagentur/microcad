use pest::iterators::Pairs;
#[allow(unused_imports)]
use pest::Parser;

use pest_derive::Parser;

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
    pub fn parse_code(pairs: Pairs<Rule>) {
        for pair in pairs {
            match pair.as_rule() {
                Rule::node_statement => {
                    let pairs = pair.into_inner();
                    println!("node statement: {pairs:#?}");
                    Self::parse_node_statement(pairs).unwrap();
                }
                _ => Self::parse_code(pair.into_inner()),
            }
        }
    }

    pub fn parse_expression(pairs: Pairs<Rule>) -> Result<Expression, ()> {
        Ok(Expression {
            literal: pairs.clone().next().unwrap().into_inner().to_string(),
        })
    }

    pub fn parse_function_argument(pairs: Pairs<Rule>) -> Result<FunctionArgument, ()> {
        Ok(FunctionArgument {
            ident: pairs.clone().next().unwrap().to_string(),
            expression: Self::parse_expression(pairs).unwrap(),
        })
    }

    pub fn parse_function_argument_list(pairs: Pairs<Rule>) -> Result<Vec<FunctionArgument>, ()> {
        let mut args = Vec::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::function_argument => {
                    args.push(Self::parse_function_argument(pair.into_inner()).unwrap());
                }
                _ => unreachable!(),
            }
        }
        Ok(args)
    }

    pub fn parse_node_statement(pairs: Pairs<Rule>) -> Result<NodeStatement, ()> {
        let mut node_statement = NodeStatement {
            ident: Default::default(),
            function_argument_list: Vec::default(),
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::function_call => return Self::parse_node_statement(pair.into_inner()),
                Rule::ident => node_statement.ident = pair.to_string(),
                Rule::function_argument_list => {
                    node_statement.function_argument_list =
                        Self::parse_function_argument_list(pair.into_inner()).unwrap()
                }
                _ => unreachable!(),
            }
        }

        Ok(node_statement)
    }
}

//include!(concat!(env!("OUT_DIR"), "/pest_test.rs"));

#[cfg(test)]
mod tests {
    use super::*;

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
                CsglParser::parse_code(pairs);
                //println!("{pairs:#?}");
            }
            Err(e) => {
                panic!("Failed parsing file in {}: {}", test_filename, e);
            }
        }
    }
}

use pest::iterators::Pairs;
#[allow(unused_imports)]
use pest::Parser;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CsglParser;

impl CsglParser {
    pub fn parse_code(pairs: Pairs<Rule>) {
        for pair in pairs {
            match pair.as_rule() {
                Rule::node_statement => {
                    println!("node statement: {pair:#?}");
                }
                _ => Self::parse_code(pair.into_inner()),
            }
        }
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

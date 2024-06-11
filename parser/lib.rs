#[allow(unused_imports)]
use pest::Parser;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CsglParser;

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r#"
            r = 5mm;
            angle = 90°;
        "#;

        match IdentParser::parse(Rule::main, input) {
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }

        let pairs = IdentParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        // Because ident_list is silent, the iterator will contain idents
        for pair in pairs {
            // A pair is a combination of the rule which matched and a span of input
            println!("Rule:    {:?}", pair.as_rule());
            println!("Span:    {:?}", pair.as_span());
            println!("Text:    {}", pair.as_str());

            /*
            // A pair can be converted to an iterator of the tokens which make it up:
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
                    Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                    _ => unreachable!(),
                };
            }*/
        }
    }
}


*/
include!(concat!(env!("OUT_DIR"), "/test.rs"));

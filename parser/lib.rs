#[allow(unused_imports)]
use pest::Parser;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CsglParser;

//include!(concat!(env!("OUT_DIR"), "/pest_test.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_nested() {
        let test_filename = "tests/nested.csg";
        let input = std::fs::read_to_string(test_filename)
            .expect(format!("Test file not found: {}", test_filename).as_str());

        match CsglParser::parse(Rule::r#code, &input) {
            Ok(_) => (),
            Err(e) => {
                panic!("Failed parsing file in {}: {}", test_filename, e);
            }
        }
    }
}

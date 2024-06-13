#[allow(unused_imports)]
use pest::Parser;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct CsglParser;

include!(concat!(env!("OUT_DIR"), "/pest_test.rs"));

fn main() {
    microcad_pest_test::generate(
        "microcad_parser::parser::Parser",
        "microcad_parser::parser::Rule",
        "../parser/grammar.pest",
    );

    if let Err(err) = microcad_markdown_test::generate("..") {
        panic!("error generating rust test code: {err}");
    }
}

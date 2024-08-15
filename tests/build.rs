fn main() {
    microcad_pest_test::generate(
        "microcad_parser::parser::Parser",
        "microcad_parser::parser::Rule",
        "../parser/grammar.pest",
    );

    microcad_markdown_test::generate("../doc").unwrap();
}

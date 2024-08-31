fn main() {
    microcad_pest_test::generate(
        "microcad_lang::parser::Parser",
        "microcad_lang::parser::Rule",
        "../lang/grammar.pest",
    );

    if let Err(err) = microcad_markdown_test::generate("..") {
        panic!("error generating rust test code: {err}");
    }
}

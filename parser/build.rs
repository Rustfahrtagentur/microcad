fn main() {
    pest_test::generate(
        "crate::parser::Parser",
        "crate::parser::Rule",
        "grammar.pest",
    );
}

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_markdown_test.rs"));
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));

#[cfg(test)]
fn eval_input(input: &str) -> microcad_core::render::Node {
    use microcad_parser::{eval::*, language::document::Document, parser};
    let doc = match parser::Parser::parse_rule::<Document>(parser::Rule::document, input) {
        Ok(doc) => doc,
        Err(err) => panic!("ERROR: {err}"),
    };

    let mut context = Context::default();
    context.add_module(microcad_std::builtin_module());

    use microcad_parser::eval::Eval;

    doc.eval(&mut context).unwrap()
}

#[cfg(test)]
fn export_yaml_for_input(input: &str, yaml_file: &str) {
    let node = eval_input(input);

    use microcad_export::Exporter;
    let mut yaml_exporter = microcad_export::yaml::YamlExporter::from_settings(
        &microcad_export::ExportSettings::with_filename(yaml_file.to_string()),
    )
    .unwrap();

    yaml_exporter.export(node).unwrap();
}

#[cfg(test)]
fn test_source_file(file: &str) {
    let mut file = std::fs::File::open(file).unwrap();

    let mut buf = String::new();
    use std::io::Read;
    file.read_to_string(&mut buf).unwrap();

    let node = eval_input(&buf);
    microcad_std::export(node.clone()).unwrap();
}

#[cfg(test)]
fn export_yaml_for_source_file(file: &str) {
    let mut file = std::fs::File::open(file).unwrap();

    let mut buf = String::new();
    use std::io::Read;
    file.read_to_string(&mut buf).unwrap();

    export_yaml_for_input(&buf, "test.yaml");
}

#[test]
fn test_algorithm_difference() {
    export_yaml_for_source_file("std/algorithm_difference.µcad");
    test_source_file("std/algorithm_difference.µcad");
}

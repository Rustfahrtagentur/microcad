// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_markdown_test.rs"));
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));

#[cfg(test)]
fn eval_input(input: &str) -> microcad_core::render::Node {
    use core::panic;
    use microcad_lang::parse::source_file::SourceFile;
    let source_file = match SourceFile::load_from_str(input) {
        Ok(doc) => doc,
        Err(err) => panic!("ERROR: {err}"),
    };

    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_std()
        .build();
    context.eval().unwrap()
}

#[cfg(test)]
fn export_yaml_for_node(node: microcad_core::render::Node, yaml_file: &str) {
    use microcad_export::Exporter;
    let mut yaml_exporter = microcad_export::yaml::YamlExporter::from_settings(
        &microcad_export::ExportSettings::with_filename(yaml_file.to_string()),
    )
    .unwrap();

    yaml_exporter.export(node).unwrap();
}

#[cfg(test)]
fn export_yaml_for_input(input: &str, yaml_file: &str) {
    let node = eval_input(input);
    export_yaml_for_node(node, yaml_file);
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
    let path = std::path::Path::new(file);
    let mut file = std::fs::File::open(file).unwrap();

    let mut buf = String::new();
    use std::io::Read;
    file.read_to_string(&mut buf).unwrap();

    // Extract filename without extension
    let filename = path.file_name().unwrap().to_str().unwrap();
    println!("Exporting YAML for {filename}");

    export_yaml_for_input(&buf, &format!("../test_output/tests/{filename}.yaml"));
}

#[test]
fn test_algorithm_difference() {
    export_yaml_for_source_file("std/algorithm_difference.µcad");
    test_source_file("std/algorithm_difference.µcad");
}

#[test]
fn test_module_implicit_init() {
    //export_yaml_for_source_file("std/module_implicit_init.µcad");
}

#[test]
fn test_context_std() {
    use core::panic;
    use microcad_lang::parse::*;
    let source_file = match SourceFile::load_from_str("use * from std;") {
        Ok(doc) => doc,
        Err(err) => panic!("ERROR: {err}"),
    };

    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_std()
        .build();

    let verify_symbol = |context: &microcad_lang::eval::Context, names| {
        use microcad_lang::parse::QualifiedName;
        let symbol = context
            .fetch_symbols_by_qualified_name(&QualifiedName(names))
            .unwrap();
        assert_eq!(symbol.len(), 1);
        symbol.first().unwrap().clone()
    };

    // Check that the context has the assert symbol
    match verify_symbol(&context, vec!["std".into(), "assert".into()]) {
        microcad_lang::eval::Symbol::BuiltinFunction(_) => {}
        _ => panic!("Expected assert symbol to be a BuiltinFunction"),
    }

    // Check that the context has the geo2d namespace
    match verify_symbol(&context, vec!["std".into(), "geo2d".into()]) {
        microcad_lang::eval::Symbol::Namespace(_) => {}
        symbol => panic!("Expected geo2d symbol to be a NamespaceDefinition {symbol:?}"),
    }

    // Check that the context has the circle symbol
    match verify_symbol(
        &context,
        vec!["std".into(), "geo2d".into(), "circle".into()],
    ) {
        microcad_lang::eval::Symbol::BuiltinModule(_) => {}
        _ => panic!("Expected circle symbol to be a BuildtinModule"),
    }

    let _ = context.eval().unwrap();

    /*
    let errors = context.diagnostics().fetch_errors();

    if !errors.is_empty() {
        for error in &errors {
            error
                .pretty_print(&mut std::io::stderr(), &context.current_source_file())
                .unwrap();
        }
        panic!("ERROR: {} errors found", errors.len());
    }*/

    // Now, after eval `use * from std` check again

    // Assert symbol, now called `assert`.
    match verify_symbol(&context, vec!["assert".into()]) {
        microcad_lang::eval::Symbol::BuiltinFunction(_) => {}
        _ => panic!("Expected assert symbol to be a BuiltinFunction"),
    }

    // geo2d namespace
    match verify_symbol(&context, vec!["geo2d".into()]) {
        microcad_lang::eval::Symbol::Namespace(_) => {}
        _ => panic!("Expected geo2d symbol to be a NamespaceDefinition"),
    }

    // circle symbol
    match verify_symbol(&context, vec!["geo2d".into(), "circle".into()]) {
        microcad_lang::eval::Symbol::BuiltinModule(_) => {}
        _ => panic!("Expected circle symbol to be a BuildtinModule"),
    }
}

#[test]
fn test_simple_module_statement() {
    use microcad_lang::parse::source_file::SourceFile;
    let source_file = match SourceFile::load_from_str("std::geo2d::circle(radius = 3.0);") {
        Ok(doc) => doc,
        Err(err) => panic!("ERROR: {err}"),
    };

    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_std()
        .build();

    let node = context.eval().unwrap();

    export_yaml_for_node(node, "../test_output/tests/simple_module_statement.yaml");
}

#[test]
fn test_simple_module_definition() {
    use microcad_lang::parse::source_file::SourceFile;
    use microcad_lang::parse::*;

    let source_file = match SourceFile::load_from_str(
        "module donut() { std::geo2d::circle(radius = 3.0); } donut();",
    ) {
        Ok(doc) => doc,
        Err(err) => panic!("ERROR: {err}"),
    };
    assert_eq!(source_file.body.len(), 2);

    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_std()
        .build();

    let _ = context.eval().unwrap();

    let module_definition = context
        .fetch_symbols_by_qualified_name(&QualifiedName(vec!["donut".into()]))
        .unwrap();
    let module_definition = match module_definition.first().unwrap() {
        microcad_lang::eval::Symbol::Module(m) => m,
        _ => panic!("Expected module definition"),
    };
    assert_eq!(module_definition.body.pre_init_statements.len(), 0);
    assert_eq!(module_definition.body.inits.len(), 1);
    assert_eq!(module_definition.body.post_init_statements.len(), 1);

    let node = module_definition
        .call(&CallArgumentList::default(), &mut context)
        .unwrap();

    context
        .diagnostics()
        .pretty_print(&mut std::io::stderr(), &context)
        .unwrap();

    if let microcad_lang::eval::Value::Node(node) = node.unwrap() {
        match *node.borrow() {
            microcad_core::render::NodeInner::Group => {}
            ref inner => panic!("Expected node to be a Group, got {:?}", inner),
        }

        export_yaml_for_node(node, "../test_output/tests/simple_module_definition.yaml");
    } else {
        panic!("Resulting value is not a node");
    }
}

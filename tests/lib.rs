// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_markdown_test.rs"));
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_source_file_test.rs"));

#[cfg(test)]
static TEST_OUTPUT_DIR: &str = "output";

#[cfg(test)]
fn eval_context(context: &mut microcad_lang::eval::Context) -> microcad_core::render::Node {
    let node = context.eval().unwrap();

    if context.diag().has_errors() {
        context
            .diag()
            .pretty_print(&mut std::io::stderr(), context)
            .unwrap();

        panic!("ERROR: {} errors found", context.diag().error_count);
    }

    node
}

/// Evaluate source input from `&str` and return the resulting node and context
#[cfg(test)]
fn eval_input_with_context(
    input: &str,
) -> (microcad_core::render::Node, microcad_lang::eval::Context) {
    use core::panic;
    use microcad_lang::parse::source_file::SourceFile;
    let source_file = match SourceFile::load_from_str(input) {
        Ok(source_file) => source_file,
        Err(err) => panic!("ERROR: {err}"),
    };

    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_std()
        .build();
    let node = eval_context(&mut context);

    (node, context)
}

#[cfg(test)]
fn eval_input(input: &str) -> microcad_core::render::Node {
    eval_input_with_context(input).0
}

#[cfg(test)]
fn export_tree_dump_for_node(node: microcad_core::render::Node, tree_dump_file: &str) {
    use microcad_export::Exporter;
    let mut tree_dump_exporter = microcad_export::tree_dump::TreeDumpExporter::from_settings(
        &microcad_export::ExportSettings::with_filename(tree_dump_file.to_string()),
    )
    .unwrap();

    tree_dump_exporter.export(node).unwrap();
}

#[cfg(test)]
fn test_source_file(file_name: &str) {
    use microcad_lang::parse::SourceFile;

    let source_file = match SourceFile::load(file_name) {
        Ok(source_file) => source_file,
        Err(err) => panic!("ERROR: {err}"),
    };
    let input_file_name = &source_file.filename.clone().unwrap();

    let output_file_name: std::path::PathBuf = [
        std::path::PathBuf::from(TEST_OUTPUT_DIR),
        input_file_name
            .strip_prefix(input_file_name.parent().unwrap())
            .unwrap()
            .to_path_buf(),
    ]
    .iter()
    .collect();

    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_std()
        .build();

    use microcad_lang::eval::*;

    // Inject `output_file` into the context as a µCAD string value `OUTPUT_FILE`
    context.add(Symbol::Value(
        "OUTPUT_FILE".into(),
        output_file_name.to_string_lossy().to_string().into(),
    ));

    let node = eval_context(&mut context);

    microcad_std::export(node.clone()).unwrap();

    let mut tree_dump_file = output_file_name;
    tree_dump_file.set_extension("tree.dump");

    export_tree_dump_for_node(node, tree_dump_file.to_str().unwrap());

    let mut ref_tree_dump_file = input_file_name.clone();
    ref_tree_dump_file.set_extension("tree.dump");

    // Compare tree dump files
    if ref_tree_dump_file.exists() {
        assert_eq!(
            std::fs::read_to_string(tree_dump_file).unwrap(),
            std::fs::read_to_string(ref_tree_dump_file).unwrap()
        );
    }
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
    export_tree_dump_for_node(
        eval_input("std::geo2d::circle(radius = 3.0);"),
        "output/simple_module_statement.tree.dump",
    );
}

#[test]
fn test_simple_module_definition() {
    use microcad_lang::parse::*;

    // Define a module `donut` with an implicit initializer `()` and call it
    let (root, mut context) = eval_input_with_context(
        r#"
        module donut() { 
            std::geo2d::circle(radius = 3.0); 
        }

        donut();
        "#,
    );

    export_tree_dump_for_node(root, "output/simple_module_definition_root.tree.dump");

    // Check the module definition
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

    // Call the module definition of `donut` and verify it
    let node = module_definition
        .call(&CallArgumentList::default(), &mut context)
        .unwrap();

    if let microcad_lang::eval::Value::Node(node) = node.unwrap() {
        match *node.borrow() {
            microcad_core::render::NodeInner::Group => {}
            ref inner => panic!("Expected node to be a Group, got {:?}", inner),
        }

        export_tree_dump_for_node(node, "output/simple_module_definition.tree.dump");
    } else {
        panic!("Resulting value is not a node");
    }
}

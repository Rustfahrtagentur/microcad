// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_export::stl::StlExporter;

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_markdown_test.rs"));
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_source_file_test.rs"));

#[cfg(test)]
static TEST_OUT_DIR: &str = "output";

// Assure `TEST_OUT_DIR` exists
#[cfg(test)]
fn make_test_out_dir() -> std::path::PathBuf {
    let test_out_dir = std::path::PathBuf::from(TEST_OUT_DIR);
    if !test_out_dir.exists() {
        std::fs::create_dir_all(&test_out_dir).unwrap();
    }
    test_out_dir
}

///Shortcut to create a ArgumentMap
#[cfg(test)]
#[macro_export]
macro_rules! args {
    ($($name:ident: $ty:ident = $value:expr),*) => {&{
        use microcad_lang::src_ref::*;
        let mut map = microcad_lang::eval::ArgumentMap::new(SrcRef(None));
        $(map.insert(stringify!($name).into(), $value.into());)*
        map
    }};
    () => {};
}

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
    let test_out_dir = make_test_out_dir();

    let source_file = match SourceFile::load(file_name) {
        Ok(source_file) => source_file,
        Err(err) => panic!("ERROR: {err}"),
    };
    let in_file_name = &source_file.filename.clone().unwrap();

    let out_file_name: std::path::PathBuf = [
        test_out_dir,
        in_file_name
            .strip_prefix(in_file_name.parent().unwrap())
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
        out_file_name.to_string_lossy().to_string().into(),
    ));

    let node = eval_context(&mut context);

    microcad_std::export(node.clone()).unwrap();

    let mut tree_dump_file = out_file_name;
    tree_dump_file.set_extension("tree.dump");

    export_tree_dump_for_node(node, tree_dump_file.to_str().unwrap());

    let mut ref_tree_dump_file = in_file_name.clone();
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
fn difference_svg() {
    use microcad_lang::eval::BuiltinModuleDefinition;
    use microcad_render::{svg::SvgRenderer, tree, Renderer2D};
    use microcad_std::{algorithm::*, geo2d::*};

    let difference = difference().unwrap();
    let group = tree::group();
    group.append(Circle::node(args!(radius: Scalar = 4.0)).unwrap());
    group.append(Circle::node(args!(radius: Scalar = 2.0)).unwrap());
    difference.append(group);

    let test_out_dir = make_test_out_dir();

    let file = std::fs::File::create(test_out_dir.join("difference.svg")).unwrap();
    let mut renderer = SvgRenderer::default();
    renderer.set_output(Box::new(file)).unwrap();
    renderer.render_node(difference).unwrap();
}

#[test]
fn difference_stl() {
    use microcad_export::stl::StlExporter;
    use microcad_lang::eval::BuiltinModuleDefinition;
    use microcad_std::algorithm;

    let difference = algorithm::difference().unwrap();
    let group = microcad_render::tree::group();
    group.append(
        microcad_std::geo3d::Cube::node(
            args!(size_x: Scalar = 4.0, size_y: Scalar = 4.0, size_z: Scalar = 4.0),
        )
        .unwrap(),
    );
    group.append(microcad_std::geo3d::Sphere::node(args!(radius: Scalar = 2.0)).unwrap());
    difference.append(group);

    let test_out_dir = make_test_out_dir();

    use microcad_export::Exporter;
    let mut exporter = StlExporter::from_settings(&microcad_core::ExportSettings::with_filename(
        test_out_dir
            .join("difference.stl")
            .to_string_lossy()
            .to_string(),
    ))
    .unwrap();

    exporter.export(difference).unwrap();
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
fn test_stl_export() {
    let test_out_dir = make_test_out_dir();

    let settings = microcad_core::ExportSettings::with_filename(
        test_out_dir
            .join("test_stl_export.stl")
            .to_string_lossy()
            .to_string(),
    );
    use microcad_export::Exporter;
    let mut exporter = StlExporter::from_settings(&settings).unwrap();

    let node = microcad_core::render::tree::root();

    use microcad_core::geo3d::*;
    let a = Manifold::cube(1.0, 1.0, 1.0);
    let b = Manifold::sphere(1.0, 32);

    let intersection: Geometry = a.intersection(&b).into();

    node.append(microcad_core::render::Node::new(
        microcad_core::render::NodeInner::Geometry3D(std::rc::Rc::new(
            intersection.fetch_mesh().into(),
        )),
    ));

    exporter.export(node).unwrap();
}

#[test]
fn test_simple_module_statement() {
    let test_out_dir = make_test_out_dir();
    let test_out_dir = test_out_dir.to_str().unwrap();

    export_tree_dump_for_node(
        eval_input("std::geo2d::circle(radius = 3.0);"),
        format!("{test_out_dir}/simple_module_statement.tree.dump").as_str(),
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

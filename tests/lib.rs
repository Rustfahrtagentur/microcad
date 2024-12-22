// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_markdown_test.rs"));
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_source_file_test.rs"));

#[cfg(test)]
static TEST_OUT_DIR: &str = "output";

#[cfg(test)]
static DEFAULT_TEST_FILE: &str = "../tests/test_cases/algorithm/difference.µcad";

#[cfg(test)]
use microcad_lang::*;

// Assure `TEST_OUT_DIR` exists
#[cfg(test)]
fn make_test_out_dir() -> std::path::PathBuf {
    let test_out_dir = std::path::PathBuf::from(TEST_OUT_DIR);
    if !test_out_dir.exists() {
        std::fs::create_dir_all(&test_out_dir).expect("test error");
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
fn eval_context(context: &mut microcad_lang::eval::Context) -> ObjectNode {
    let node = context.eval();
    let node = node.expect("test error");
    println!("{node:?}");

    if context.diag().has_errors() {
        context
            .diag()
            .pretty_print(&mut std::io::stderr(), context)
            .expect("test error");
    }

    node
}

/// Evaluate source input from `&str` and return the resulting node and context
#[cfg(test)]
fn eval_input_with_context(input: &str) -> (ObjectNode, microcad_lang::eval::Context) {
    use core::panic;
    use microcad_lang::parse::source_file::SourceFile;
    let source_file = match SourceFile::load_from_str(input) {
        Ok(source_file) => source_file,
        Err(err) => panic!("ERROR: {err}"),
    };

    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_builtin()
        .expect("builtin error")
        .build();
    let node = eval_context(&mut context);

    (node, context)
}

#[cfg(test)]
fn eval_input(input: &str) -> ObjectNode {
    eval_input_with_context(input).0
}

#[cfg(test)]
fn export_tree_dump_for_node(node: ObjectNode, tree_dump_file: &str) {
    use microcad_export::Exporter;
    let mut tree_dump_exporter = microcad_export::tree_dump::TreeDumpExporter::from_settings(
        &microcad_export::ExportSettings::with_filename(tree_dump_file.to_string()),
    )
    .expect("test error");

    tree_dump_exporter.export(node).expect("test error");
}

#[cfg(test)]
fn test_source_file(file_name: &str) {
    use microcad_lang::parse::SourceFile;
    let test_out_dir = make_test_out_dir();

    let source_file = match SourceFile::load(file_name) {
        Ok(source_file) => source_file,
        Err(err) => panic!("ERROR: {err}"),
    };
    let in_file_name = &source_file.filename.clone().expect("test error");

    let out_file_name: std::path::PathBuf = [
        test_out_dir,
        in_file_name
            .strip_prefix("test_cases")
            .expect("test error")
            .to_path_buf(),
    ]
    .iter()
    .collect();

    let out_dir = out_file_name.parent().expect("test error");
    if !out_dir.exists() {
        std::fs::create_dir_all(out_dir).expect("test error");
    }

    eprintln!("Testing: {in_file_name:?} -> {out_file_name:?}");

    let hash = source_file.hash();

    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_std("../std")
        .expect("no std found")
        .build();

    let std_symbol = context.fetch(&"std".into()).expect("test error");
    match std_symbol.as_ref() {
        microcad_lang::eval::Symbol::Namespace(_) => {
            //println!("{namespace:#?}");
        }
        _ => panic!("Expected symbol to be a Namespace"),
    }

    use parse::GetSourceFileByHash;
    assert!(context.get_source_file_by_hash(hash).is_some());

    use microcad_lang::eval::*;

    // Inject `output_file` into the context as a µcad string value `OUTPUT_FILE`
    context.add(Symbol::Value(
        "OUTPUT_FILE".into(),
        out_file_name.to_string_lossy().to_string().into(),
    ));

    let node = eval_context(&mut context);

    microcad_std::export(node.clone()).expect("test error");

    let mut tree_dump_file = out_file_name.clone();
    tree_dump_file.set_extension("tree.dump");

    export_tree_dump_for_node(node, tree_dump_file.to_str().expect("test error"));

    let mut ref_tree_dump_file = in_file_name.clone();
    ref_tree_dump_file.set_extension("tree.dump");

    // Compare tree dump files
    if ref_tree_dump_file.exists() {
        assert_eq!(
            std::fs::read_to_string(tree_dump_file)
                .expect("test error")
                .replace("\r\n", "\n")
                .trim(),
            std::fs::read_to_string(ref_tree_dump_file)
                .expect("test error")
                .replace("\r\n", "\n")
                .trim()
        );
    }

    // Create log file containing the diagnostics
    let mut log_file = out_file_name;
    log_file.set_extension("log");
    let mut log = std::fs::File::create(log_file).expect("test error");

    context
        .diag()
        .pretty_print(&mut log, &context)
        .expect("test error");
}

#[test]
fn test_diag_list() {
    use microcad_lang::diag::*;
    use microcad_lang::eval::EvalError;
    use microcad_lang::parse::GetSourceFileByHash;

    let source_file =
        crate::parse::SourceFile::load(DEFAULT_TEST_FILE).expect("Could not load source file");

    let mut diagnostics = DiagList::default();

    let mut body_iter = source_file.body.iter();

    diagnostics.info(
        body_iter.next().expect("test error"),
        "This is an info".to_string(),
    );
    diagnostics
        .warning(
            body_iter.next().expect("test error"),
            Box::new(EvalError::CustomError("This is a warning".into())),
        )
        .expect("test error");
    diagnostics
        .error(
            body_iter.next().expect("test error"),
            Box::new(EvalError::CustomError("This is an error".into())),
            None,
        )
        .expect("test error");

    assert_eq!(diagnostics.len(), 3);
    let mut output = std::io::Cursor::new(Vec::new());
    diagnostics
        .pretty_print(
            &mut output,
            source_file
                .get_source_file_by_hash(source_file.hash())
                .expect("test error"),
        )
        .expect("test error");

    // Hol den Inhalt des Puffers
    let result = String::from_utf8(output.into_inner()).expect("Invalid UTF-8");
    assert_eq!(
        result,
        format!(
            "info: This is an info
  ---> {DEFAULT_TEST_FILE}:1:1
     |
   1 | use std::*;
     | ^^^^^^^^^^^
     |
warning: This is a warning
  ---> {DEFAULT_TEST_FILE}:4:1
     |
   4 | export(\"{{OUTPUT_FILE}}.stl\") algorithm::difference() {{
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
error: This is an error
  ---> {DEFAULT_TEST_FILE}:10:1
     |
  10 | export(\"{{OUTPUT_FILE}}.svg\") algorithm::difference() {{
     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
     |
"
        )
        .to_string()
    );
}

#[test]
fn difference_svg() {
    use microcad_export::svg::SvgRenderer;
    use microcad_lang::eval::BuiltinModuleDefinition;
    use microcad_std::{algorithm::*, geo2d::*};

    let difference = difference().expect("test error");
    let group = objecttree::group();
    group.append(Circle::node(args!(radius: Scalar = 4.0)).expect("test error"));
    group.append(Circle::node(args!(radius: Scalar = 2.0)).expect("test error"));
    difference.append(group);

    let test_out_dir = make_test_out_dir();

    let file = std::fs::File::create(test_out_dir.join("difference.svg")).expect("test error");
    let mut renderer = SvgRenderer::default();
    renderer.set_output(Box::new(file)).expect("test error");

    let difference = objecttree::bake2d(&mut renderer, difference).expect("test error");

    use microcad_core::geo2d::Renderer;
    renderer.render_node(difference).expect("test error");
}

#[test]
fn difference_stl() {
    use microcad_export::stl::StlExporter;
    use microcad_lang::eval::BuiltinModuleDefinition;
    use microcad_std::algorithm;

    let difference = algorithm::difference().expect("test error");
    let group = objecttree::group();
    group.append(
        microcad_std::geo3d::Cube::node(
            args!(size_x: Scalar = 4.0, size_y: Scalar = 4.0, size_z: Scalar = 4.0),
        )
        .expect("test error"),
    );
    group.append(
        microcad_std::geo3d::Sphere::node(args!(radius: Scalar = 2.0)).expect("test error"),
    );
    difference.append(group);

    let test_out_dir = make_test_out_dir();

    use microcad_export::Exporter;
    let mut exporter = StlExporter::from_settings(&microcad_core::ExportSettings::with_filename(
        test_out_dir
            .join("difference.stl")
            .to_string_lossy()
            .to_string(),
    ))
    .expect("test error");

    exporter.export(difference).expect("test error");
}

#[test]
fn test_context_builtin() {
    use core::panic;
    use microcad_lang::parse::*;
    let source_file = match SourceFile::load_from_str("use __builtin::*;") {
        Ok(doc) => doc,
        Err(err) => panic!("ERROR: {err:?}"),
    };

    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_builtin()
        .expect("builtin error")
        .build();

    let verify_symbol = |context: &mut microcad_lang::eval::Context, names| {
        use microcad_lang::parse::QualifiedName;
        let symbol = &context
            .fetch_symbols_by_qualified_name(&QualifiedName(names))
            .expect("test error");
        assert_eq!(symbol.len(), 1);
        symbol.first().expect("test error").clone()
    };

    // Check that the context has the assert symbol
    match verify_symbol(&mut context, vec!["__builtin".into(), "assert".into()]) {
        microcad_lang::eval::Symbol::BuiltinFunction(_) => {}
        _ => panic!("Expected assert symbol to be a BuiltinFunction"),
    }

    // Check that the context has the geo2d namespace
    match verify_symbol(&mut context, vec!["__builtin".into(), "geo2d".into()]) {
        microcad_lang::eval::Symbol::Namespace(_) => {}
        symbol => panic!("Expected geo2d symbol to be a NamespaceDefinition {symbol:?}"),
    }

    // Check that the context has the circle symbol
    match verify_symbol(
        &mut context,
        vec!["__builtin".into(), "geo2d".into(), "circle".into()],
    ) {
        microcad_lang::eval::Symbol::BuiltinModule(_) => {}
        _ => panic!("Expected circle symbol to be a BuiltinModule"),
    }

    let _ = context.eval().expect("test error");

    // Now, after eval `use * from std` check again

    // Assert symbol, now called `assert`.
    match verify_symbol(&mut context, vec!["assert".into()]) {
        microcad_lang::eval::Symbol::BuiltinFunction(_) => {}
        _ => panic!("Expected assert symbol to be a BuiltinFunction"),
    }

    // geo2d namespace
    match verify_symbol(&mut context, vec!["geo2d".into()]) {
        microcad_lang::eval::Symbol::Namespace(_) => {}
        _ => panic!("Expected geo2d symbol to be a NamespaceDefinition"),
    }

    // circle symbol
    match verify_symbol(&mut context, vec!["geo2d".into(), "circle".into()]) {
        microcad_lang::eval::Symbol::BuiltinModule(_) => {}
        _ => panic!("Expected circle symbol to be a BuildtinModule"),
    }
}

#[test]
fn test_simple_module_statement() {
    let test_out_dir = make_test_out_dir();
    let test_out_dir = test_out_dir.to_str().expect("test error");

    export_tree_dump_for_node(
        eval_input("__builtin::geo2d::circle(radius = 3.0);"),
        format!("{test_out_dir}/simple_module_statement.tree.dump").as_str(),
    );
}

#[test]
fn test_reexport_symbols() {
    let (_, context) = eval_input_with_context(
        r#"
        pub use __builtin::assert;
        "#,
    );

    let mut symbol_table = microcad_lang::eval::SymbolTable::default();
    use microcad_lang::eval::Symbols;
    context.copy(&mut symbol_table);

    assert!(symbol_table.fetch(&"assert".into()).is_some());

    println!("{symbol_table:#?}");
}

#[test]
fn test_reexport_symbols_inside_namespace() {
    let (_, context) = eval_input_with_context(
        r#"
        namespace test {
            pub use __builtin::assert;
        }
        "#,
    );
    use microcad_lang::eval::Symbols;
    assert!(context.fetch(&"test".into()).is_some());

    let test_namespace = context.fetch(&"test".into()).expect("test error");
    match test_namespace.as_ref() {
        microcad_lang::eval::Symbol::Namespace(namespace) => {
            assert!(namespace.fetch(&"assert".into()).is_some());
        }
        _ => panic!("Expected symbol to be a Namespace"),
    }
}

#[test]
fn test_simple_module_definition() {
    use microcad_lang::parse::*;

    // Define a module `donut` with an implicit initializer `()` and call it
    let (root, mut context) = eval_input_with_context(
        r#"
        module donut() { 
            __builtin::geo2d::circle(radius = 3.0); 
        }

        donut();
        "#,
    );

    export_tree_dump_for_node(root, "output/simple_module_definition_root.tree.dump");

    // Check the module definition
    let module_definition = context
        .fetch_symbols_by_qualified_name(&QualifiedName(vec!["donut".into()]))
        .expect("test error");
    let module_definition = match module_definition.first().expect("test error") {
        microcad_lang::eval::Symbol::Module(m) => m,
        _ => panic!("Expected module definition"),
    };
    assert_eq!(module_definition.body.pre_init_statements.len(), 1);
    assert_eq!(module_definition.body.explicit_inits.len(), 0);
    assert!(module_definition.body.implicit_init.is_some());
    assert_eq!(module_definition.body.post_init_statements.len(), 0);

    // Call the module definition of `donut` and verify it
    let nodes = module_definition
        .call(&CallArgumentList::default(), &mut context)
        .expect("test error");

    if let Some(node) = nodes.first() {
        match *node.borrow() {
            ObjectNodeInner::Group(_) => {}
            ref inner => panic!("Expected node to be a Group, got {:?}", inner),
        }

        export_tree_dump_for_node(node.clone(), "output/simple_module_definition.tree.dump");
    } else {
        panic!("Resulting value is not a node");
    }
}

#[test]
fn test_module_definition_with_parameters() {
    use microcad_lang::parse::*;

    // Define a module `donut` with an implicit initializer `()` and call it
    let (root, mut context) = eval_input_with_context(
        r#"
        module donut(radius: Scalar) { 
            __builtin::geo2d::circle(radius); 
        }

        donut(radius = 3.0);
        donut(radius = 5.0);

        // Test if we can access the radius parameter
        __builtin::assert(donut(radius = 4.0).radius == 4.0);
        "#,
    );

    export_tree_dump_for_node(
        root,
        "output/module_definition_with_parameters_root.tree.dump",
    );

    // Check the module definition
    let module_definition = context
        .fetch_symbols_by_qualified_name(&QualifiedName(vec!["donut".into()]))
        .expect("test error");
    let module_definition = match module_definition.first().expect("test error") {
        microcad_lang::eval::Symbol::Module(m) => m,
        _ => panic!("Expected module definition"),
    };
    assert_eq!(module_definition.body.pre_init_statements.len(), 1);
    assert_eq!(module_definition.body.explicit_inits.len(), 0);
    assert!(module_definition.body.implicit_init.is_some());
    assert_eq!(module_definition.body.post_init_statements.len(), 0);

    // Call the module definition of `donut` and verify it
    use crate::parser::*;

    let nodes = module_definition
        .call(
            &Parser::parse_rule::<CallArgumentList>(Rule::call_argument_list, "radius = 6.0", 0)
                .expect("test error"),
            &mut context,
        )
        .expect("test error");

    if let Some(node) = nodes.first() {
        match *node.borrow() {
            ObjectNodeInner::Group(ref symbols) => {
                use microcad_lang::eval::*;
                let symbol = symbols.fetch(&"radius".into()).expect("test error");
                match symbol.as_ref() {
                    Symbol::Value(_, value) => {
                        assert_eq!(value, &6.0.into());
                    }
                    _ => panic!("Expected symbol to be a Value"),
                }
            }
            ref inner => panic!("Expected node to be a Group, got {:?}", inner),
        }

        export_tree_dump_for_node(
            node.clone(),
            "output/module_definition_with_parameters.tree.dump",
        );
    } else {
        panic!("Resulting value is not a node");
    }
}

#[test]
fn module_definition_init() {
    use microcad_lang::parse::*;

    // Define a module `donut` with an implicit initializer `()` and call it
    let (root, mut context) = eval_input_with_context(
        r#"
        module circle {
            pre_init_statement = 0;

            init(r: Scalar) {
                radius = r;
            }

            init(d: Scalar) {
                radius = d / 2.0;
            }

            __builtin::geo2d::circle(radius);
        }

        circle(r = 3.0);
        circle(d = 6.0);
        "#,
    );

    export_tree_dump_for_node(root, "output/module_definition_init_root.tree.dump");

    // Check the module definition
    let module_definition = context
        .fetch_symbols_by_qualified_name(&QualifiedName(vec!["circle".into()]))
        .expect("test error");
    let module_definition = match module_definition.first().expect("test error") {
        microcad_lang::eval::Symbol::Module(m) => m,
        _ => panic!("Expected module definition"),
    };
    assert_eq!(module_definition.body.pre_init_statements.len(), 1);
    assert_eq!(module_definition.body.explicit_inits.len(), 2);
    assert_eq!(module_definition.body.post_init_statements.len(), 1);

    // Call the module definition of `donut` and verify it
    use crate::parser::*;

    let nodes = module_definition
        .call(
            &Parser::parse_rule::<CallArgumentList>(Rule::call_argument_list, "r = 6.0", 0)
                .expect("test error"),
            &mut context,
        )
        .expect("test error");

    if let Some(node) = nodes.first() {
        match *node.borrow() {
            ObjectNodeInner::Group(ref symbols) => {
                use microcad_lang::eval::*;
                let symbol = symbols.fetch(&"radius".into()).expect("test error");
                match symbol.as_ref() {
                    Symbol::Value(_, value) => {
                        assert_eq!(value, &6.0.into());
                    }
                    _ => panic!("Expected symbol to be a Value"),
                }
            }
            ref inner => panic!("Expected node to be a Group, got {:?}", inner),
        }

        export_tree_dump_for_node(
            node.clone(),
            "output/module_definition_with_parameters.tree.dump",
        );
    } else {
        panic!("Resulting value is not a node");
    }
}

#[test]
fn test_module_src_ref() {
    // Define a module `donut` with an implicit initializer `()` and call it
    let (root, _) = eval_input_with_context(
        r#"
        module donut { 
            init(d: Scalar) {
                radius = d / 2.0;
                __builtin::geo2d::circle(radius); 
            }
        }

        donut(d = 3.0);
        "#,
    );
    export_tree_dump_for_node(root, "output/test_module_src_ref.tree.dump");
}

#[test]
fn test_load_std() {
    use microcad_lang::parse::*;

    let std_source_file = match SourceFile::load("../std/std.µcad") {
        Ok(std_source_file) => std_source_file,
        Err(err) => panic!("ERROR: {err:?}"),
    };

    let mut context = microcad_std::ContextBuilder::new(std_source_file)
        .with_builtin()
        .expect("builtin error")
        .build();

    let namespace = context
        .current_source_file()
        .expect("test error")
        .eval_as_namespace(&mut context, "std".into())
        .expect("test error");

    if context.diag().has_errors() {
        context
            .diag()
            .pretty_print(&mut std::io::stdout(), &context)
            .expect("test error");
        panic!("ERROR: {} errors found", context.diag().error_count);
    }

    // Test if assert symbol works
    assert!(namespace.fetch(&"assert".into()).is_some());

    // Check if we have the algorithm namespace
    assert!(namespace.fetch(&"algorithm".into()).is_some());

    let source_file = match SourceFile::load_from_str(
        "
        use std::*;
        geo2d::circle(r = 2.0mm);
        geo2d::circle(d = 2.0mm);
        geo2d::circle(radius = 2.0mm);
        geo2d::circle(diameter = 2.0mm);
        
        use std::geo2d::*;

        geo3d::cube(size_x = 2.0mm, size_y = 2.0mm, size_z = 2.0mm);
        
        ",
    ) {
        Ok(doc) => doc,
        Err(err) => panic!("ERROR: {err:?}"),
    };

    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_builtin()
        .expect("builtin error")
        .build();

    use microcad_lang::eval::*;
    context.add(Symbol::Namespace(namespace));

    let node = context.eval().expect("test error");

    export_tree_dump_for_node(node, "output/test_load_std.tree.dump");
}

#[test]
fn test_with_std() {
    use microcad_lang::parse::*;

    let source_file = match SourceFile::load_from_str(
        "
        use std::*;
        geo2d::circle(r = 2.0mm);
        geo2d::circle(d = 2.0mm);
        geo2d::circle(radius = 2.0mm);
        geo2d::circle(diameter = 2.0mm);
        
        use std::geo2d::*;

        geo3d::cube(size_x = 2.0mm, size_y = 2.0mm, size_z = 2.0mm);
        
        ",
    ) {
        Ok(doc) => doc,
        Err(err) => panic!("ERROR: {err:?}"),
    };

    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_std("../std")
        .expect("no std found")
        .build();

    let node = context.eval().expect("test error");

    export_tree_dump_for_node(node, "output/test_with_std.tree.dump");
}

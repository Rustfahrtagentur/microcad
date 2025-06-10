// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

/// Global test initialization.
#[cfg(test)]
#[ctor::ctor]
fn init() {
    env_logger::init();
}

#[cfg(test)]
use microcad_lang::{eval::*, parser::*, resolve::*, src_ref::*, syntax::*};

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_source_file_test.rs"));

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_markdown_test.rs"));

/// Evaluate the context for a file in the `test_cases` folder.
#[cfg(test)]
fn context_for_file(filename: &str) -> microcad_lang::eval::Context {
    use microcad_lang::eval::*;

    let filename = format!("../tests/test_cases/{filename}");
    Context::from_source(&filename, microcad_builtin::builtin_module(), &[]).expect(&filename)
}

/// Generate a reference filename from `filename` with `extension`.
///
/// `syntax/multiplicity.µcad` -> `syntax/multiplicity.log`
#[cfg(test)]
fn reference_filename(filename: &str, extension: &str) -> String {
    let mut path = std::path::PathBuf::from(filename);
    path.set_extension(extension);
    path.to_string_lossy().into_owned()
}

/// Generate an output filename from `filename` with `extension`.
///
/// `syntax/multiplicity.µcad` -> `./test/syntax_multiplicity.log`
#[cfg(test)]
fn output_filename(filename: &str, extension: &str) -> String {
    let filename = filename.replace(['/', '\\'], "_");
    format!(
        "../tests/.test/{}.{}",
        filename.trim_end_matches(".µcad"),
        extension
    )
}

/// Compare the content of two files.
#[cfg(test)]
fn compare_files(file1: &str, file2: &str) {
    let content1 =
        std::fs::read_to_string(file1).unwrap_or_else(|_| panic!("Failed to read file: {}", file1));
    let content2 =
        std::fs::read_to_string(file2).unwrap_or_else(|_| panic!("Failed to read file: {}", file2));
    assert_eq!(
        content1, content2,
        "File contents differ: {} vs {}",
        file1, file2
    );
}

/// Compares the output to reference file.
#[cfg(test)]
fn compare_output_to_reference_file(filename: &str, extension: &str) {
    let output = &output_filename(filename, extension);
    let reference = &reference_filename(filename, extension);

    use std::path::Path;
    match (Path::new(output).exists(), Path::new(reference).exists()) {
        (true, true) => compare_files(output, reference),
        (true, false) => log::info!(r#"No reference file "{reference}""#),
        (false, true) => {
            log::error!(r#"There is reference file "{reference}", but no output file "{output}"!"#)
        }
        (false, false) => unreachable!("This should not happen."),
    }
}

/// Write the content of a `Display` to a file.
#[cfg(test)]
fn write_and_compare(value: impl std::fmt::Display, filename: &str, extension: &str) {
    // Write value to file
    {
        let output_filename = output_filename(filename, extension);
        let file = std::fs::File::create(&output_filename)
            .unwrap_or_else(|_| panic!("Failed to create file: {}", output_filename));
        use std::io::Write;
        let mut writer = std::io::BufWriter::new(file);
        write!(writer, "{}", value).expect("Valid file write")
    }

    // Compare
    {
        compare_output_to_reference_file(filename, extension);
    }
}

/// This function is called for every `.µcad` file in `./test_cases`.
///
/// It will evaluate a `.µcad` file and generate:
/// * `.symbol_tree`: A dump of the root symbol tree.
/// * `.model_tree`: A dump of the model node tree.
/// * `.log`: An error log file.
/// * (TODO) `.svg`: An output SVG file, if the test produces a 2D geometry.
/// * (TODO) `.stl`: An output STL file, if the test produces a 3D geometry.
///
/// All output files will be stored in `./.test`.
///
/// You can place `.log`, `.model_tree` and other reference files next to the respective `.µcad` files.
#[cfg(test)]
fn test_source_file(filename: &str) {
    let source_file =
        SourceFile::load(format!("../tests/test_cases/{filename}")).expect("Valid source file");

    let mut context =
        Context::from_source_captured(source_file, microcad_builtin::builtin_module(), &[]);

    let symbol_tree = context.root();
    write_and_compare(symbol_tree, filename, "symbol_tree");

    match context.eval() {
        Ok(model_tree) => {
            write_and_compare(model_tree, filename, "model_tree");
        }
        Err(err) => {
            panic!("Error evaluating context: {err}");
        }
    }

    if let Some(output) = context.output() {
        write_and_compare(output, filename, "log");
    }
}

/// Helper function to create a qualified name from &str
#[cfg(test)]
fn qualified_name(s: &str) -> QualifiedName {
    QualifiedName::no_ref(
        s.split("::")
            .map(|x| Identifier(microcad_lang::src_ref::Refer::none(x.into())))
            .collect(),
    )
}

/// Helper function to create a call argument list from &str
#[cfg(test)]
fn call_argument_value_list(s: &str, context: &mut Context) -> CallArgumentValueList {
    let call_argument_list = Parser::parse_rule::<CallArgumentList>(
        microcad_lang::parser::Rule::call_argument_list,
        s,
        0,
    )
    .expect("Valid CallArgumentList");

    call_argument_list
        .eval(context)
        .expect("Valid call argument list")
}

#[test]
fn modules() {
    assert!(context_for_file("syntax/module.µcad").eval().is_ok());
}

#[test]
fn scopes() {
    assert!(context_for_file("syntax/scopes.µcad").eval().is_ok());
}

#[test]
fn context_with_symbols() {
    let mut context = context_for_file("syntax/call.µcad");

    context
        .lookup(
            &"__builtin::assert_valid"
                .try_into()
                .expect("unexpected name error"),
        )
        .expect("symbol not found");
    context
        .lookup(
            &"__builtin::assert_invalid"
                .try_into()
                .expect("unexpected name error"),
        )
        .expect("symbol not found");

    assert!(context.eval().is_ok());
}

#[test]
fn part_implicit_init_call() {
    use microcad_lang::*;

    let mut context = context_for_file("syntax/part/implicit_init.µcad");

    let node = context.lookup(&qualified_name("a")).expect("Node expected");

    // Check node id
    if let Ok(node) =
        context.lookup(&Identifier(microcad_lang::src_ref::Refer::none("a".into())).into())
    {
        let id = node.id();
        assert_eq!(id.id(), "a");
    }

    // Get part definition for symbol `a`
    let definition = match &node.borrow().def {
        SymbolDefinition::Part(definition) => definition.clone(),
        _ => panic!("Symbol is not a part"),
    };

    // Call `a` with `b = 3.0`
    let nodes = definition
        .call(
            &call_argument_value_list("b = 3.0", &mut context),
            &mut context,
        )
        .expect("Valid nodes");

    assert_eq!(nodes.len(), 1, "There should be one node");

    fn check_node_property_b(node: &model_tree::ModelNode, value: f64) {
        if let model_tree::Element::Object(ref object) = *node.borrow().element() {
            assert_eq!(
                object
                    .get_property_value(&Identifier(Refer::none("b".into())))
                    .expect("Property `b`"),
                &value::Value::Scalar(value)
            );
        } else {
            panic!("Object node expected")
        }
    }

    // Test if resulting object node has property `b` with value `3.0`
    check_node_property_b(nodes.first().expect("Node expected"), 3.0);

    // Call `a` with `b = [1.0, 2.0]` (multiplicity)
    let nodes = definition
        .call(
            &call_argument_value_list("b = [1.0, 2.0]", &mut context),
            &mut context,
        )
        .expect("Valid nodes");

    assert_eq!(nodes.len(), 2, "There should be two nodes");

    check_node_property_b(nodes.first().expect("Node expected"), 1.0);
    check_node_property_b(nodes.get(1).expect("Node expected"), 2.0);
}

#[test]
fn part_explicit_init_call() {
    use microcad_lang::diag::Diag;
    use microcad_lang::*;

    let mut context = context_for_file("syntax/part/explicit_init.µcad");
    let node = context
        .lookup(&qualified_name("circle"))
        .expect("Node expected");

    // Get part definition for symbol `a`
    let definition = match &node.borrow().def {
        SymbolDefinition::Part(definition) => definition.clone(),
        _ => panic!("Symbol is not a part"),
    };

    // Helper function to check if the object node contains a property radius with specified value
    fn check_node_property_radius(node: &model_tree::ModelNode, value: f64) {
        if let model_tree::Element::Object(ref object) = *node.borrow().element() {
            log::trace!("Object: {object}");
            assert_eq!(
                object
                    .get_property_value(&Identifier::no_ref("radius"))
                    .expect("Property `radius`"),
                &value::Value::Scalar(value)
            );
        } else {
            panic!("Object node expected")
        }
    }

    // Call `circle(radius = 3.0)`
    {
        let nodes = definition
            .call(
                &call_argument_value_list("radius = 3.0", &mut context),
                &mut context,
            )
            .expect("A valid value");
        assert_eq!(nodes.len(), 1, "There should be one node");
        check_node_property_radius(nodes.first().expect("Node expected"), 3.0);
    }

    // Call `circle(r = 3.0)`
    {
        let nodes = definition
            .call(
                &call_argument_value_list("r = 3.0", &mut context),
                &mut context,
            )
            .expect("Valid nodes");
        assert_eq!(nodes.len(), 1, "There should be one node");
        check_node_property_radius(nodes.first().expect("Node expected"), 3.0);
    }

    // Call circle(d = 6.0)`
    {
        let nodes = definition
            .call(
                &call_argument_value_list("d = 6.0", &mut context),
                &mut context,
            )
            .expect("Valid nodes");
        assert_eq!(nodes.len(), 1, "There should be one node");
        check_node_property_radius(nodes.first().expect("Node expected"), 3.0);
    }

    // Call `circle(d = [1.0, 2.0])` (multiplicity)
    {
        let nodes = definition
            .call(
                &call_argument_value_list("d = [1.0, 2.0]", &mut context),
                &mut context,
            )
            .expect("Valid nodes");
        assert_eq!(nodes.len(), 2, "There should be two nodes");
        check_node_property_radius(nodes.first().expect("Node expected"), 0.5);
        check_node_property_radius(nodes.get(1).expect("Node expected"), 1.0);
    }

    // Call `circle()` (missing arguments)
    {
        let nodes = definition
            .call(&CallArgumentValueList::default(), &mut context)
            .expect("Valid nodes");
        assert_eq!(nodes.len(), 0, "There should no nodes");
        log::trace!("{}", context.diagnosis());
    }
}

#[test]
fn model_node_output() {
    let mut context = context_for_file("syntax/children_marker.µcad");

    let node = context.eval().expect("No error");

    log::info!("Tree:\n{node}");
    assert!(node.has_children());
}

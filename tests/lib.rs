// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
mod use_test;

#[cfg(test)]
use log::debug;

#[cfg(test)]
use microcad_lang::{parser::*, resolve::*, syntax::*};

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));
/*
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_source_file_test.rs"));
*/
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_markdown_test.rs"));

#[cfg(test)]
fn evaluate_file(filename: &str) -> microcad_lang::eval::EvalContext {
    use microcad_lang::eval::*;

    let filename = format!("../tests/test_cases/{filename}");
    EvalContext::from_source(&filename, microcad_builtin::builtin_namespace(), &[])
        .expect(&filename)
}

#[test]
fn namespaces() {
    assert!(evaluate_file("syntax/namespace.µcad").eval().is_ok());
}

#[test]
fn scopes() {
    assert!(evaluate_file("../tests/test_cases/syntax/scopes.µcad")
        .eval()
        .is_ok());
}

#[test]
fn context_with_symbols() {
    let mut context = evaluate_file("../tests/test_cases/syntax/call.µcad");

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

/// Helper function to create a qualified name from &str
#[cfg(test)]
fn qualified_name(s: &str) -> QualifiedName {
    QualifiedName(
        s.split("::")
            .map(|x| Identifier(microcad_lang::src_ref::Refer::none(x.into())))
            .collect(),
    )
}

/// Helper function to create a call argument list from &str
#[cfg(test)]
fn call_argument_list(s: &str) -> CallArgumentList {
    Parser::parse_rule::<CallArgumentList>(microcad_lang::parser::Rule::call_argument_list, s, 0)
        .expect("Valid CallArgumentList")
}

#[test]
fn module_implicit_init_call() {
    use microcad_lang::*;
    env_logger_init();

    let mut context = evaluate_file("syntax/module/implicit_init.µcad");
    debug!("Source File:\n{}", context.get_root());

    let node = context
        .fetch_global(&qualified_name("implicit_init::a"))
        .expect("Node expected");

    // Check node id
    if let Ok(node) =
        context.lookup(&Identifier(microcad_lang::src_ref::Refer::none("a".into())).into())
    {
        let id = node.borrow().id();
        assert_eq!(id, "a");
    }

    // Get module definition for symbol `a`
    let module_definition = match &node.borrow().def {
        SymbolDefinition::Module(module_definition) => module_definition.clone(),
        _ => panic!("Symbol is not a module"),
    };

    // Call module `a` with `b = 3.0`
    let nodes = module_definition
        .eval_call(&call_argument_list("b = 3.0"), &mut context)
        .expect("Valid nodes")
        .fetch_nodes();

    assert_eq!(nodes.len(), 1, "There should be one node");

    fn check_node_property_b(node: &objects::ObjectNode, value: f64) {
        if let objects::ObjectNodeInner::Object(ref object) = *node.borrow() {
            assert_eq!(
                object
                    .get_property_value(&"b".into())
                    .expect("Property `b`"),
                &value::Value::Scalar(src_ref::Refer::none(value))
            );
        } else {
            panic!("Object node expected")
        }
    }

    // Test if resulting object node has property `b` with value `3.0`
    check_node_property_b(nodes.first().expect("Node expected"), 3.0);

    // Call module `a` with `b = [1.0, 2.0]` (multiplicity)
    let nodes = module_definition
        .eval_call(&call_argument_list("b = [1.0, 2.0]"), &mut context)
        .expect("Valid nodes")
        .fetch_nodes();

    assert_eq!(nodes.len(), 2, "There should be two nodes");

    check_node_property_b(nodes.first().expect("Node expected"), 1.0);
    check_node_property_b(nodes.get(1).expect("Node expected"), 2.0);
}

#[test]
fn module_explicit_init_call() {
    use microcad_lang::*;
    env_logger_init();

    let mut context = evaluate_file("syntax/module/explicit_init.µcad");
    let node = context
        .fetch_global(&qualified_name("explicit_init::circle"))
        .expect("Node expected");

    // Get module definition for symbol `a`
    let module_definition = match &node.borrow().def {
        SymbolDefinition::Module(module_definition) => module_definition.clone(),
        _ => panic!("Symbol is not a module"),
    };

    // Helper function to check if the object node contains a property radius with specified value
    fn check_node_property_radius(node: &objects::ObjectNode, value: f64) {
        if let objects::ObjectNodeInner::Object(ref object) = *node.borrow() {
            assert_eq!(
                object
                    .get_property_value(&"radius".into())
                    .expect("Property `radius`"),
                &value::Value::Scalar(src_ref::Refer::none(value))
            );
        } else {
            panic!("Object node expected")
        }
    }

    // Call module `circle(radius = 3.0)`
    {
        let nodes = module_definition
            .eval_call(&call_argument_list("radius = 3.0"), &mut context)
            .expect("A valid value")
            .fetch_nodes();
        assert_eq!(nodes.len(), 1, "There should be one node");
        check_node_property_radius(nodes.first().expect("Node expected"), 3.0);
    }

    // Call module `circle(r = 3.0)`
    {
        let nodes = module_definition
            .eval_call(&call_argument_list("r = 3.0"), &mut context)
            .expect("Valid nodes")
            .fetch_nodes();
        assert_eq!(nodes.len(), 1, "There should be one node");
        check_node_property_radius(nodes.first().expect("Node expected"), 3.0);
    }

    // Call module `circle(d = 6.0)`
    {
        let nodes = module_definition
            .eval_call(&call_argument_list("d = 6.0"), &mut context)
            .expect("Valid nodes")
            .fetch_nodes();
        assert_eq!(nodes.len(), 1, "There should be one node");
        check_node_property_radius(nodes.first().expect("Node expected"), 3.0);
    }

    // Call module `circle(d = [1.0, 2.0])` (multiplicity)
    {
        let nodes = module_definition
            .eval_call(&call_argument_list("d = [1.0, 2.0]"), &mut context)
            .expect("Valid nodes")
            .fetch_nodes();
        assert_eq!(nodes.len(), 2, "There should be two nodes");
        check_node_property_radius(nodes.first().expect("Node expected"), 0.5);
        check_node_property_radius(nodes.get(1).expect("Node expected"), 1.0);
    }

    // Call module `circle()` (missing arguments)
    {
        let nodes = module_definition
            .eval_call(&CallArgumentList::default(), &mut context)
            .expect("Valid nodes")
            .fetch_nodes();
        assert_eq!(nodes.len(), 0, "There should no nodes");
        context
            .diag_handler()
            .pretty_print(&mut std::io::stdout(), &context)
            .expect("Valid formatter");
    }
}

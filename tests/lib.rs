// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
use log::debug;

#[cfg(test)]
use microcad_lang::{
    parser::Parser,
    syntax::QualifiedName,
    resolve::SymbolDefinition,
    syntax::{CallArgumentList, Identifier},
};

#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_pest_test.rs"));
/*
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_source_file_test.rs"));
*/
#[cfg(test)]
include!(concat!(env!("OUT_DIR"), "/microcad_markdown_test.rs"));

#[cfg(test)]
fn load_source_file(
    filename: &str,
) -> (
    std::rc::Rc<microcad_lang::syntax::SourceFile>,
    microcad_lang::eval::EvalContext,
) {
    use microcad_builtin::*;
    use microcad_lang::{eval::*, resolve::Resolve, syntax::*};
    let source_file = SourceFile::load(format!("../tests/test_cases/{filename}"))
        .expect("cannot load test file: {filename}");
    let symbols = source_file.resolve(None);

    let mut context = EvalContext::new(
        symbols.clone(),
        microcad_builtin::builtin_namespace(),
        vec![],
        None,
    );
    context.add_symbol(builtin_namespace());

    assert!(source_file.eval(&mut context).is_ok());

    (source_file, context)
}

#[test]
fn namespaces() {
    use microcad_lang::eval::*;

    let (source_file, mut context) = load_source_file("syntax/namespace.µcad");

    //println!("{}", symbol_node.borrow());

    assert!(source_file.eval(&mut context).is_ok());
}

#[test]
fn scopes() {
    use microcad_builtin::*;
    use microcad_lang::{eval::*, syntax::*};

    let source_file =
        SourceFile::load("../tests/test_cases/syntax/scopes.µcad").expect("cannot load test file");

    let mut context = EvalContext::from_source_file(
        source_file.clone(),
        microcad_builtin::builtin_namespace(),
        vec![],
    );
    context.add_symbol(builtin_namespace());

    assert!(source_file.eval(&mut context).is_ok());
}

#[test]
fn context_with_symbols() {
    use microcad_builtin::*;
    use microcad_lang::{eval::*, syntax::*};
    let source_file =
        SourceFile::load("../tests/test_cases/syntax/call.µcad").expect("cannot load test file");
    let mut context = EvalContext::from_source_file(
        source_file.clone(),
        microcad_builtin::builtin_namespace(),
        vec![],
    );

    context.add_symbol(builtin_namespace());
    context
        .fetch_global(
            &"__builtin::assert_valid"
                .try_into()
                .expect("unexpected name error"),
        )
        .expect("symbol not found");
    context
        .fetch_global(
            &"__builtin::assert_invalid"
                .try_into()
                .expect("unexpected name error"),
        )
        .expect("symbol not found");
    let eval = source_file.eval(&mut context);
    assert!(eval.is_ok());
}


/// Helper function to create a qualified name from &str
#[cfg(test)]
fn qualified_name(s: &str) -> QualifiedName {
    QualifiedName(s.split("::").map(|x| Identifier(microcad_lang::src_ref::Refer::none(x.into()))).collect())
}

/// Helper function to create a call argument list from &str
#[cfg(test)]
fn call_argument_list(s: &str) -> CallArgumentList {
    Parser::parse_rule::<CallArgumentList>(
        microcad_lang::parser::Rule::call_argument_list,
        s,
        0,
    )
    .expect("Valid CallArgumentList")
}

#[test]
fn module_implicit_init_call() {
    use microcad_lang::*;
    env_logger_init();

    let (source_file, mut context) = load_source_file("syntax/module/implicit_init.µcad");
    debug!("Source File:\n{}", source_file);

    let node = context.fetch_global(&qualified_name("implicit_init::a")).expect("Node expected");

    // Check node id
    {
        let id = node.borrow().id();
        assert_eq!(id, "a");    
    }

    // Get module definition for symbol `a`
    let module_definition = match &node.borrow().def {
        SymbolDefinition::Module(module_definition) => module_definition.clone(),
        _ => panic!("Symbol is not a module")
    };

    // Call module `a` with `b = 3.0`
    let nodes = module_definition
        .eval_call(&call_argument_list("b = 3.0"), &mut context)
        .expect("Valid nodes").fetch_nodes();

    assert_eq!(nodes.len(), 1, "There should be one node");

    fn check_node_property_b(node: &objects::ObjectNode, value: f64) {
        if let objects::ObjectNodeInner::Object(ref object) = *node.borrow() {
            assert_eq!(object.get_property_value(&"b".into()).expect("Property `b`"), &value::Value::Scalar(src_ref::Refer::none(value)));
        } else {
            panic!("Object node expected")
        }
    }

    // Test if resulting object node has property `b` with value `3.0`
    check_node_property_b(nodes.first().expect("Node expected"), 3.0);

    // Call module `a` with `b = [1.0, 2.0]` (multiplicity)
    let nodes = module_definition
        .eval_call(&call_argument_list("b = [1.0, 2.0]"), &mut context)
        .expect("Valid nodes").fetch_nodes();

    assert_eq!(nodes.len(), 2, "There should be two nodes");

    check_node_property_b(nodes.first().expect("Node expected"), 1.0);
    check_node_property_b(nodes.get(1).expect("Node expected"), 2.0);
}

#[test]
fn module_explicit_init_call() {
    use microcad_lang::*;
    env_logger_init();

    let (_, mut context) = load_source_file("syntax/module/explicit_init.µcad");
    let node = context.fetch_global(&qualified_name("explicit_init::circle")).expect("Node expected");

    // Get module definition for symbol `a`
    let module_definition = match &node.borrow().def {
        SymbolDefinition::Module(module_definition) => module_definition.clone(),
        _ => panic!("Symbol is not a module")
    };

    // Helper function to check if the object node contains a property radius with specified value
    fn check_node_property_radius(node: &objects::ObjectNode, value: f64) {
        if let objects::ObjectNodeInner::Object(ref object) = *node.borrow() {
            assert_eq!(object.get_property_value(&"radius".into()).expect("Property `radius`"), &value::Value::Scalar(src_ref::Refer::none(value)));
        } else {
            panic!("Object node expected")
        }
    }
    
    // Call module `circle(radius = 3.0)`
    {
        let nodes = module_definition
            .eval_call(&call_argument_list("radius = 3.0"), &mut context)
            .expect("A valid value").fetch_nodes();
        assert_eq!(nodes.len(), 1, "There should be one node");
        check_node_property_radius(nodes.first().expect("Node expected"), 3.0);
    }

    // Call module `circle(r = 3.0)`
    {
        let nodes = module_definition
            .eval_call(&call_argument_list("r = 3.0"), &mut context)
            .expect("Valid nodes").fetch_nodes();
        assert_eq!(nodes.len(), 1, "There should be one node");
        check_node_property_radius(nodes.first().expect("Node expected"), 3.0);
    }
        
    // Call module `circle(d = 6.0)`
    {
        let nodes = module_definition
            .eval_call(&call_argument_list("d = 6.0"), &mut context)
            .expect("Valid nodes").fetch_nodes();
        assert_eq!(nodes.len(), 1, "There should be one node");
        check_node_property_radius(nodes.first().expect("Node expected"), 3.0);
    }
    
    // Call module `circle(d = [1.0, 2.0])` (multiplicity)
    {
        let nodes = module_definition
            .eval_call(&call_argument_list("d = [1.0, 2.0]"), &mut context)
            .expect("Valid nodes").fetch_nodes();
        assert_eq!(nodes.len(), 2, "There should be two nodes");
        check_node_property_radius(nodes.first().expect("Node expected"), 0.5);
        check_node_property_radius(nodes.get(1).expect("Node expected"), 1.0);
    }

    // Call module `circle()` (missing arguments)
    {
        let nodes = module_definition
            .eval_call(&CallArgumentList::default(), &mut context)
            .expect("Valid nodes").fetch_nodes();
        assert_eq!(nodes.len(), 0, "There should no nodes");
        context.diag_handler().pretty_print(&mut std::io::stdout(), &context).expect("Valid formatter");
    }

}

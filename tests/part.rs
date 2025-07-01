// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
use microcad_lang::{
    GetPropertyValue, eval::*, model_tree::*, resolve::*, src_ref::*, syntax::*, ty::*, value::*,
};

/// Helper function to create a qualified name from &str
#[cfg(test)]
fn qualified_name(s: &str) -> QualifiedName {
    QualifiedName::no_ref(
        s.split("::")
            .map(|x| Identifier(Refer::none(x.into())))
            .collect(),
    )
}

/// Helper function to create a argument list from &str
#[cfg(test)]
fn call_argument_value_list(s: &str, context: &mut Context) -> ArgumentValueList {
    use microcad_lang::parser::*;

    let argument_list =
        Parser::parse_rule::<ArgumentList>(Rule::argument_list, s, 0).expect("Valid ArgumentList");

    argument_list.eval(context).expect("Valid argument list")
}

#[test]
fn workbench_call() {
    let mut context = crate::context_for_file("syntax/workbench/plan.µcad");

    let node = context.lookup(&qualified_name("a")).expect("Node expected");

    // Check node id
    if let Ok(node) = context.lookup(&Identifier(Refer::none("a".into())).into()) {
        let id = node.id();
        assert_eq!(id.id(), "a");
    }

    // Get workbench definition for symbol `a`
    let definition = match &node.borrow().def {
        SymbolDefinition::Workbench(definition) => definition.clone(),
        _ => panic!("Symbol is not a workbench"),
    };

    // Call `a` with `b = 3.0`
    let nodes = definition
        .call(
            &call_argument_value_list("b = 3.0", &mut context),
            &mut context,
        )
        .expect("Valid nodes");

    assert_eq!(nodes.len(), 1, "There should be one node");

    fn check_node_property_b(node: &ModelNode, value: f64) {
        if let Element::Object(ref object) = *node.borrow().element() {
            assert_eq!(
                object.get_property_value(&Identifier(Refer::none("b".into()))),
                Value::Quantity(Quantity::new(value, QuantityType::Scalar))
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
fn workbench_initializer_call() {
    use microcad_lang::diag::Diag;
    use microcad_lang::*;

    let mut context = crate::context_for_file("syntax/workbench/initializer.µcad");
    let node = context
        .lookup(&qualified_name("circle"))
        .expect("Node expected");

    // Get workbench definition for symbol `a`
    let definition = match &node.borrow().def {
        SymbolDefinition::Workbench(definition) => definition.clone(),
        _ => panic!("Symbol is not a workbench"),
    };

    // Helper function to check if the object node contains a property radius with specified value
    fn check_node_property_radius(node: &model_tree::ModelNode, value: f64) {
        if let model_tree::Element::Object(ref object) = *node.borrow().element() {
            log::trace!("Object: {object}");
            assert_eq!(
                object.get_property_value(&Identifier::no_ref("radius")),
                Value::Quantity(Quantity::new(value, ty::QuantityType::Scalar))
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
            .call(&ArgumentValueList::default(), &mut context)
            .expect("Valid nodes");
        assert_eq!(nodes.len(), 0, "There should no nodes");
        log::trace!("{}", context.diagnosis());
    }
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
use microcad_lang::{eval::*, model::*, resolve::*, src_ref::*, syntax::*, ty::*, value::*};

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

    let symbol = context
        .lookup(&qualified_name("a"))
        .expect("Symbol expected");

    // Check symbol id
    if let Ok(symbol) = context.lookup(&Identifier(Refer::none("a".into())).into()) {
        let id = symbol.id();
        assert_eq!(id.id(), "a");
    }

    // Get workbench definition for symbol `a`
    let definition = match &symbol.borrow().def {
        SymbolDefinition::Workbench(definition) => definition.clone(),
        _ => panic!("Symbol is not a workbench"),
    };

    // Call `a` with `b = 3.0`
    let models = definition
        .call(
            &call_argument_value_list("b = 3.0", &mut context),
            &mut context,
        )
        .expect("Valid models");

    assert_eq!(models.len(), 1, "There should be one model");

    fn check_property_b(model: &Model, value: f64) {
        assert_eq!(
            *model
                .borrow()
                .get_property(&Identifier(Refer::none("b".into())))
                .expect("property"),
            Value::Quantity(Quantity::new(value, QuantityType::Scalar))
        );
    }

    // Test if resulting object model has property `b` with value `3.0`
    check_property_b(models.first().expect("Model expected"), 3.0);

    // Call `a` with `b = [1.0, 2.0]` (multiplicity)
    let models = definition
        .call(
            &call_argument_value_list("b = [1.0, 2.0]", &mut context),
            &mut context,
        )
        .expect("Valid models");

    assert_eq!(models.len(), 2, "There should be two models");

    check_property_b(models.first().expect("Model expected"), 1.0);
    check_property_b(models.get(1).expect("Model expected"), 2.0);
}

#[test]
fn workbench_initializer_call() {
    use microcad_lang::diag::Diag;
    use microcad_lang::*;

    let mut context = crate::context_for_file("syntax/workbench/initializer.µcad");
    let symbol = context
        .lookup(&qualified_name("Circle"))
        .expect("Symbol expected");

    // Get workbench definition for symbol `a`
    let definition = match &symbol.borrow().def {
        SymbolDefinition::Workbench(definition) => definition.clone(),
        _ => panic!("Symbol is not a workbench"),
    };

    // Helper function to check if the object model contains a property radius with specified value
    fn check_property_radius(model: &model::Model, value: f64) {
        assert_eq!(
            *model
                .borrow()
                .get_property(&Identifier(Refer::none("radius".into())))
                .expect("property"),
            Value::Quantity(Quantity::new(value, ty::QuantityType::Scalar))
        );
    }

    // Call `Circle(radius = 3.0)`
    {
        let models = definition
            .call(
                &call_argument_value_list("radius = 3.0", &mut context),
                &mut context,
            )
            .expect("A valid value");
        assert_eq!(models.len(), 1, "There should be one model");
        check_property_radius(models.first().expect("Model expected"), 3.0);
    }

    // Call `Circle(r = 3.0)`
    {
        let models = definition
            .call(
                &call_argument_value_list("r = 3.0", &mut context),
                &mut context,
            )
            .expect("Valid models");
        assert_eq!(models.len(), 1, "There should be one model");
        check_property_radius(models.first().expect("Model expected"), 3.0);
    }

    // Call Circle(d = 6.0)`
    {
        let models = definition
            .call(
                &call_argument_value_list("d = 6.0", &mut context),
                &mut context,
            )
            .expect("Valid models");
        assert_eq!(models.len(), 1, "There should be one model");
        check_property_radius(models.first().expect("Model expected"), 3.0);
    }

    // Call `Circle(d = [1.0, 2.0])` (multiplicity)
    {
        let models = definition
            .call(
                &call_argument_value_list("d = [1.0, 2.0]", &mut context),
                &mut context,
            )
            .expect("Valid models");
        assert_eq!(models.len(), 2, "There should be two models");
        check_property_radius(models.first().expect("Model expected"), 0.5);
        check_property_radius(models.get(1).expect("Model expected"), 1.0);
    }

    // Call `Circle()` (missing arguments)
    {
        let models = definition
            .call(&ArgumentValueList::default(), &mut context)
            .expect("Valid models");
        assert_eq!(models.len(), 0, "There should no models");
        log::trace!("{}", context.diagnosis());
    }
}

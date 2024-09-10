// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD function parser entities

mod function_body;
mod function_definition;
mod function_signature;
mod function_statement;

pub use function_body::*;
pub use function_definition::*;
pub use function_signature::*;
pub use function_statement::*;

#[test]
fn assignment() {
    use crate::{eval::*, parse::*, parser::*};

    let assignment = Parser::parse_rule_or_panic::<Assignment>(Rule::assignment, "a = 1");

    let mut context = Context::default();

    assert_eq!(&assignment.name, "a");
    assert_eq!(
        assignment.value.eval(&mut context).unwrap().to_string(),
        "1"
    );
    assert!(assignment.specified_type.is_none());

    assignment.eval(&mut context).unwrap();

    assert_eq!(
        context
            .find_symbols(&"a".into())
            .first()
            .unwrap()
            .id()
            .unwrap(),
        "a"
    );
}

#[test]
fn function_signature() {
    use crate::eval::Ty;
    use crate::{parser::*, r#type::Type};

    let input = "(a: scalar, b: scalar) -> scalar";

    let function_signature =
        Parser::parse_rule_or_panic::<FunctionSignature>(Rule::function_signature, input);

    assert_eq!(function_signature.parameters.len(), 2);
    assert_eq!(function_signature.return_type.unwrap().ty(), Type::Scalar);
}

#[test]
fn function_definition() {
    use crate::parser::*;

    let input = "function test(a: scalar, b: scalar) -> scalar {
            c = 1.0;
            return a + b + c;
        }";
    Parser::parse_rule_or_panic::<std::rc::Rc<FunctionDefinition>>(Rule::function_definition, input);
}

#[test]
fn function_evaluate() {
    use crate::{eval::*, parse::*, parser::*};

    let input = r#"
        function test(a: scalar, b: scalar) -> scalar {
            c = 1.0;
            return a + b + c;
        }"#;

    let function_def = Parser::parse_rule_or_panic::<std::rc::Rc<FunctionDefinition>>(
        Rule::function_definition,
        input,
    );

    let mut context = Context::default();
    context.add_function(function_def);

    let input = "test(a = 1.0, b = 2.0)";
    let expr = Parser::parse_rule_or_panic::<Expression>(Rule::expression, input);

    let value = expr.eval(&mut context).unwrap();
    assert_eq!(value.to_string(), "4");
}


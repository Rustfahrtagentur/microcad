// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad function parser entities

mod function_definition;
mod function_signature;

pub use function_definition::*;
pub use function_signature::*;

#[test]
fn assignment() {
    use crate::{parse::*, parser::*};

    let assignment =
        Parser::parse_rule::<Assignment>(Rule::assignment, "a = 1", 0).expect("test error");

    let mut context = EvalContext::default();

    assert_eq!(&assignment.name, "a");
    assert_eq!(
        assignment
            .value
            .eval(&mut context)
            .expect("test error")
            .to_string(),
        "1"
    );
    assert!(assignment.specified_type.is_none());

    assignment.eval(&mut context).expect("test error");

    assert_eq!(
        context
            .fetch(&"a".into())
            .expect("test error")
            .id()
            .expect("test error"),
        "a"
    );
}

#[test]
fn function_signature() {
    use crate::Ty;
    use crate::{parser::*, r#type::Type};

    let input = "(a: Scalar, b: Scalar) -> Scalar";

    let function_signature =
        Parser::parse_rule::<FunctionSignature>(Rule::function_signature, input, 0)
            .expect("test error");

    assert_eq!(function_signature.parameters.len(), 2);
    assert_eq!(
        function_signature.return_type.expect("test error").ty(),
        Type::Scalar
    );
}

#[test]
fn function_definition() {
    use crate::parser::*;

    let input = "function test(a: Scalar, b: Scalar) -> Scalar {
            c = 1.0;
            return a + b + c;
        }";
    Parser::parse_rule::<std::rc::Rc<FunctionDefinition>>(Rule::function_definition, input, 0)
        .expect("test error");
}

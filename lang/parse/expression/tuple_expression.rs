// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tuple expression

use crate::{parse::*, parser::*, r#type::*, src_ref::*};

/// TODO: maybe CallArgumentList should be `ArgumentList` and get independent of module `call`?
type ArgumentList = CallArgumentList;

/// Tuple expression
#[derive(Clone, Debug, Default)]
pub struct TupleExpression {
    /// List of tuple members
    pub args: ArgumentList,
    /// Common unit
    pub unit: Option<Unit>,
    /// `true` if this is a named tuple
    pub is_named: bool,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for TupleExpression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for TupleExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        let call_argument_list =
            CallArgumentList::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;
        if call_argument_list.is_empty() {
            return Err(ParseError::EmptyTupleExpression);
        }

        // Count number of positional and named arguments
        let named_count: usize = call_argument_list
            .iter()
            .map(|c| if c.name.is_some() { 1 } else { 0 })
            .sum();

        if named_count > 0 && named_count < call_argument_list.len() {
            return Err(ParseError::MixedTupleArguments);
        }

        Ok(TupleExpression {
            is_named: named_count == call_argument_list.len(),
            args: call_argument_list,
            unit: match inner.next() {
                Some(pair) => Some(Unit::parse(pair)?),
                None => None,
            },
            src_ref: pair.clone().into(),
        })
    }
}

impl std::fmt::Display for TupleExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({})",
            self.args
                .iter()
                .map(|arg| if self.is_named {
                    format!(
                        "{} = {}",
                        arg.name.clone().expect(INTERNAL_PARSE_ERROR),
                        arg.value
                    )
                } else {
                    arg.to_string()
                })
                .collect::<Vec<String>>()
                .join(", ")
        )?;
        if let Some(unit) = self.unit {
            write!(f, "{}", unit)?;
        }
        Ok(())
    }
}

#[test]
fn unnamed_tuple() {
    let input = "(1.0, 2.0, 3.0)mm";
    let expr = Parser::parse_rule::<TupleExpression>(Rule::tuple_expression, input, 0)
        .expect("test error");
    let mut context = EvalContext::default();
    let value = expr.eval(&mut context).expect("test error");
    assert_eq!(
        value.ty(),
        Type::UnnamedTuple(UnnamedTupleType(vec![
            Type::Length,
            Type::Length,
            Type::Length
        ]))
    );
}

#[test]
fn test_named_tuple() {
    let input = "(a = 1.0, b = 2.0, c = 3.0)mm";
    let expr = Parser::parse_rule::<TupleExpression>(Rule::tuple_expression, input, 0)
        .expect("test error");
    let mut context = EvalContext::default();
    let value = expr.eval(&mut context).expect("test error");
    assert_eq!(
        value.ty(),
        Type::NamedTuple(NamedTupleType(
            vec![
                ("a".into(), Type::Length),
                ("b".into(), Type::Length),
                ("c".into(), Type::Length),
            ]
            .into_iter()
            .collect()
        ))
    );
}

#[test]
fn test_vec2() {
    let input = "(x = 1mm, y = 1mm)";
    let expr = Parser::parse_rule::<TupleExpression>(Rule::tuple_expression, input, 0)
        .expect("test error");
    let mut context = EvalContext::default();
    let value = expr.eval(&mut context).expect("test error");
    assert_eq!(value.ty(), Type::Vec2);
}

#[test]
fn test_vec3() {
    let input = "(x = 1, y = 2, z = 3)mm";
    let expr = Parser::parse_rule::<TupleExpression>(Rule::tuple_expression, input, 0)
        .expect("test error");
    let mut context = EvalContext::default();
    let value = expr.eval(&mut context).expect("test error");
    assert_eq!(value.ty(), Type::Vec3);
}

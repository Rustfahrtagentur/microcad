// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tuple expression

use crate::{eval::*, parse::*, parser::*, r#type::*, src_ref::*};

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
        let call_argument_list = CallArgumentList::parse(inner.next().unwrap())?;
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
                .map(|c| if self.is_named {
                    format!("{} = {}", c.name.clone().unwrap(), c.value)
                } else {
                    c.to_string()
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

impl Eval for TupleExpression {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> crate::eval::Result<Value> {
        if !self.is_named {
            // Unnamed tuple
            let mut value_list = ValueList::new(Vec::new(), self.args.src_ref());
            for arg in self.args.iter() {
                value_list.push(arg.value.eval(context)?);
            }
            if let Some(unit) = self.unit {
                value_list.add_unit_to_unitless(unit)?;
            }
            Ok(Value::UnnamedTuple(UnnamedTuple::new(value_list)))
        } else {
            // Named tuple
            let mut map = std::collections::BTreeMap::new();
            for (ident, expr) in self
                .args
                .iter()
                .map(|c| (c.name.clone().unwrap(), c.value.clone()))
            {
                let mut value = expr.clone().eval(context)?;
                if let Some(unit) = self.unit {
                    value.add_unit_to_unitless(unit)?;
                }
                map.insert(ident.clone(), value);
            }

            let (x, y, z) = (&"x".into(), &"y".into(), &"z".into());

            use microcad_core::{Vec2, Vec3};

            match (map.len(), map.values().all(|v| v.ty() == Type::Length)) {
                // Special case for Vec2: if we have exactly two lengths with names "x" and "y", we can create a Vec2
                (2, true) => {
                    if let (Some(x), Some(y)) = (map.get(x), map.get(y)) {
                        return Ok(Value::Vec2(Refer::new(
                            Vec2::new(x.try_into()?, y.try_into()?),
                            SrcRef::merge(x, y),
                        )));
                    }
                }
                // Special case for Vec3: if we have exactly three lengths with names "x", "y" and "z", we can create a Vec3
                (3, true) => {
                    if let (Some(x), Some(y), Some(z)) = (map.get(x), map.get(y), map.get(z)) {
                        return Ok(Value::Vec3(Refer::new(
                            Vec3::new(x.try_into()?, y.try_into()?, z.try_into()?),
                            SrcRef::merge(x, z),
                        )));
                    }
                }
                _ => {}
            }

            Ok(Value::NamedTuple(NamedTuple::new(map, self.src_ref())))
        }
    }
}

#[test]
fn unnamed_tuple() {
    let input = "(1.0, 2.0, 3.0)mm";
    let expr = Parser::parse_rule::<TupleExpression>(Rule::tuple_expression, input, 0).unwrap();
    let mut context = Context::default();
    let value = expr.eval(&mut context).unwrap();
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
    let expr = Parser::parse_rule::<TupleExpression>(Rule::tuple_expression, input, 0).unwrap();
    let mut context = Context::default();
    let value = expr.eval(&mut context).unwrap();
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
    let expr = Parser::parse_rule::<TupleExpression>(Rule::tuple_expression, input, 0).unwrap();
    let mut context = Context::default();
    let value = expr.eval(&mut context).unwrap();
    assert_eq!(value.ty(), Type::Vec2);
}

#[test]
fn test_vec3() {
    let input = "(x = 1, y = 2, z = 3)mm";
    let expr = Parser::parse_rule::<TupleExpression>(Rule::tuple_expression, input, 0).unwrap();
    let mut context = Context::default();
    let value = expr.eval(&mut context).unwrap();
    assert_eq!(value.ty(), Type::Vec3);
}

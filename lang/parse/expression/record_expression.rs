// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Record expression

use crate::{eval::*, parse::*, parser::*, r#type::*, src_ref::*, sym::*};

/// TODO: maybe CallArgumentList should be `ArgumentList` and get independent of module `call`?
type ArgumentList = CallArgumentList;

/// Record expression
#[derive(Clone, Debug, Default)]
pub struct RecordExpression {
    /// List of record members
    pub args: ArgumentList,
    /// Common unit
    pub unit: Option<Unit>,
    /// `true` if this is a named record
    pub is_named: bool,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for RecordExpression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for RecordExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        let call_argument_list =
            CallArgumentList::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?;
        if call_argument_list.is_empty() {
            return Err(ParseError::EmptyRecordExpression);
        }

        // Count number of positional and named arguments
        let named_count: usize = call_argument_list
            .iter()
            .map(|c| if c.name.is_some() { 1 } else { 0 })
            .sum();

        if named_count > 0 && named_count < call_argument_list.len() {
            return Err(ParseError::MixedRecordArguments);
        }

        Ok(RecordExpression {
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

impl std::fmt::Display for RecordExpression {
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

impl Eval for RecordExpression {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> crate::eval::EvalResult<Value> {
        if self.is_named {
            // Named record
            let mut map = std::collections::BTreeMap::new();
            for (ident, expr) in self
                .args
                .iter()
                .map(|c| (c.name.clone().expect(INTERNAL_PARSE_ERROR), c.value.clone()))
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

            Ok(Value::NamedRecord(NamedRecord::new(map, self.src_ref())))
        } else {
            // Unnamed record
            let mut value_list = ValueList::new(Vec::new(), self.args.src_ref());
            for arg in self.args.iter() {
                value_list.push(arg.value.eval(context)?);
            }
            if let Some(unit) = self.unit {
                value_list.add_unit_to_unitless(unit)?;
            }
            Ok(Value::UnnamedRecord(UnnamedRecord::new(value_list)))
        }
    }
}

#[test]
fn unnamed_record() {
    let input = "(1.0, 2.0, 3.0)mm";
    let expr = Parser::parse_rule::<RecordExpression>(Rule::record_expression, input, 0)
        .expect("test error");
    let mut context = Context::default();
    let value = expr.eval(&mut context).expect("test error");
    assert_eq!(
        value.ty(),
        Type::UnnamedRecord(UnnamedRecordType(vec![
            Type::Length,
            Type::Length,
            Type::Length
        ]))
    );
}

#[test]
fn test_named_record() {
    let input = "(a = 1.0, b = 2.0, c = 3.0)mm";
    let expr = Parser::parse_rule::<RecordExpression>(Rule::record_expression, input, 0)
        .expect("test error");
    let mut context = Context::default();
    let value = expr.eval(&mut context).expect("test error");
    assert_eq!(
        value.ty(),
        Type::NamedRecord(NamedRecordType(
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
    let expr = Parser::parse_rule::<RecordExpression>(Rule::record_expression, input, 0)
        .expect("test error");
    let mut context = Context::default();
    let value = expr.eval(&mut context).expect("test error");
    assert_eq!(value.ty(), Type::Vec2);
}

#[test]
fn test_vec3() {
    let input = "(x = 1, y = 2, z = 3)mm";
    let expr = Parser::parse_rule::<RecordExpression>(Rule::record_expression, input, 0)
        .expect("test error");
    let mut context = Context::default();
    let value = expr.eval(&mut context).expect("test error");
    assert_eq!(value.ty(), Type::Vec3);
}

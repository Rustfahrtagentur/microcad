use crate::{eval::*, language::*, parser::*, with_pair_ok};
use microcad_core::*;

#[derive(Clone, Debug, Default)]
pub struct TupleExpression {
    args: CallArgumentList,
    unit: Option<Unit>,
    is_named: bool,
}

impl TupleExpression {
    pub fn args(&self) -> &CallArgumentList {
        &self.args
    }

    pub fn unit(&self) -> Option<&Unit> {
        self.unit.as_ref()
    }

    pub fn is_named(&self) -> bool {
        self.is_named
    }

    pub fn is_unnamed(&self) -> bool {
        !self.is_named
    }
}

impl Parse for TupleExpression {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut inner = pair.clone().into_inner();
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

        with_pair_ok!(
            TupleExpression {
                args: call_argument_list.value().clone(),
                unit: match inner.next() {
                    Some(pair) => Some(*Unit::parse(pair)?),
                    None => None,
                },
                is_named: named_count == call_argument_list.len(),
            },
            pair
        )
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

    fn eval(&self, context: &mut Context) -> Result<Value, EvalError> {
        if self.is_unnamed() {
            // Unnamed tuple
            let mut value_list = ValueList::new();
            for arg in self.args.iter() {
                let value = arg.value.eval(context)?;
                value_list.push(value);
            }
            if let Some(unit) = self.unit {
                value_list.add_unit_to_unitless_types(unit)?;
            }
            Ok(Value::UnnamedTuple(value_list.into()))
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
                    value.add_unit_to_unitless_types(unit)?;
                }
                map.insert(ident.clone(), value);
            }

            let (x_ident, y_ident, z_ident) = (&"x".into(), &"y".into(), &"z".into());

            match (map.len(), map.values().all(|v| v.ty() == Type::Length)) {
                // Special case for Vec2: if we have exactly two lengths with names "x" and "y", we can create a Vec2
                (2, true) => {
                    if let (Some(x), Some(y)) = (map.get(x_ident), map.get(y_ident)) {
                        return Ok(Value::Vec2(Vec2::new(x.try_into()?, y.try_into()?)));
                    }
                }
                // Special case for Vec3: if we have exactly three lengths with names "x", "y" and "z", we can create a Vec3
                (3, true) => {
                    if let (Some(x), Some(y), Some(z)) =
                        (map.get(x_ident), map.get(y_ident), map.get(z_ident))
                    {
                        return Ok(Value::Vec3(Vec3::new(
                            x.try_into()?,
                            y.try_into()?,
                            z.try_into()?,
                        )));
                    }
                }
                _ => {}
            }

            Ok(Value::NamedTuple(map.into()))
        }
    }
}

#[test]
fn unnamed_tuple() {
    let input = "(1.0, 2.0, 3.0)mm";
    let expr = Parser::parse_rule_or_panic::<TupleExpression>(Rule::tuple_expression, input);
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
    let expr = Parser::parse_rule_or_panic::<TupleExpression>(Rule::tuple_expression, input);
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
    let expr = Parser::parse_rule_or_panic::<TupleExpression>(Rule::tuple_expression, input);
    let mut context = Context::default();
    let value = expr.eval(&mut context).unwrap();
    assert_eq!(value.ty(), Type::Vec2);
}

#[test]
fn test_vec3() {
    let input = "(x = 1, y = 2, z = 3)mm";
    let expr = Parser::parse_rule_or_panic::<TupleExpression>(Rule::tuple_expression, input);
    let mut context = Context::default();
    let value = expr.eval(&mut context).unwrap();
    assert_eq!(value.ty(), Type::Vec3);
}

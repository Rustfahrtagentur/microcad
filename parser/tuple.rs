use std::collections::BTreeMap;

use crate::call::{self, CallArgumentList};
use crate::eval::{Context, Eval};
use crate::langtype::{Ty, Type};
use crate::parser::{Pair, Parse, ParseError};
use crate::units::Unit;
use crate::value::{NamedTuple, UnnamedTuple, Value, ValueList, Vec2, Vec3};

#[derive(Default, Clone)]
pub struct TupleExpression(CallArgumentList, Option<Unit>);

impl Parse for TupleExpression {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();
        let call_argument_list = CallArgumentList::parse(pairs.next().unwrap())?;
        if call_argument_list.is_empty() {
            return Err(ParseError::EmptyTupleExpression);
        }
        if call_argument_list.contains_positional() && call_argument_list.contains_named() {
            return Err(ParseError::MixedTupleArguments);
        }

        Ok(TupleExpression(
            call_argument_list,
            match pairs.next() {
                Some(pair) => Some(Unit::parse(pair)?),
                None => None,
            },
        ))
    }
}

impl Eval for TupleExpression {
    fn eval(self, context: Option<&Context>) -> Result<Value, crate::eval::Error> {
        if self.0.contains_positional() {
            // Unnamed tuple
            let mut value_list = ValueList::new();
            for expr in self.0.get_positional() {
                let value = expr.clone().eval(context)?;
                value_list.push(value);
            }
            if let Some(unit) = self.1 {
                value_list.add_unit_to_scalar_types(unit);
            }
            Ok(Value::UnnamedTuple(UnnamedTuple(value_list)))
        } else {
            // Named tuple
            let mut map = BTreeMap::new();
            for (ident, expr) in self.0.get_named() {
                let mut value = expr.clone().eval(context)?;
                if let Some(unit) = self.1 {
                    value.add_unit_to_scalar_types(unit);
                }
                map.insert(ident.clone(), value);
            }

            let (x_ident, y_ident, z_ident) = (&"x".into(), &"y".into(), &"z".into());

            match (map.len(), map.values().all(|v| v.ty() == Type::Length)) {
                // Special case for Vec2: if we have exactly two lengths with names "x" and "y", we can create a Vec2
                (2, true) => {
                    if let (Some(x), Some(y)) = (map.get(x_ident), map.get(y_ident)) {
                        return Ok(Value::Vec2(Vec2::new(x.into_scalar()?, y.into_scalar()?)));
                    }
                }
                // Special case for Vec3: if we have exactly three lengths with names "x", "y" and "z", we can create a Vec3
                (3, true) => {
                    if let (Some(x), Some(y), Some(z)) =
                        (map.get(x_ident), map.get(y_ident), map.get(z_ident))
                    {
                        return Ok(Value::Vec3(Vec3::new(
                            x.into_scalar()?,
                            y.into_scalar()?,
                            z.into_scalar()?,
                        )));
                    }
                }
                _ => {}
            }

            Ok(Value::NamedTuple(NamedTuple(map)))
        }
    }

    fn eval_type(&self, context: Option<&Context>) -> Result<Type, crate::eval::Error> {
        if self.0.contains_positional() {
            // Unnamed tuple
            let mut types = Vec::new();
            for expr in self.0.get_positional() {
                let value = expr.clone().eval(context)?;
                types.push(value.ty());
            }
            Ok(Type::UnnamedTuple(crate::langtype::UnnamedTupleType(types)))
        } else {
            // Named tuple

            let mut map = BTreeMap::new();
            for (ident, expr) in self.0.get_named() {
                map.insert(ident.clone(), expr.eval_type(context)?);
            }

            let (x_ident, y_ident, z_ident) = (&"x".into(), &"y".into(), &"z".into());

            match (map.len(), map.values().all(|ty| *ty == Type::Length)) {
                // Special case for Vec2: if we have exactly two lengths with names "x" and "y", we can create a Vec2
                (2, true) => {
                    if let (Some(_), Some(_)) = (map.get(x_ident), map.get(y_ident)) {
                        return Ok(Type::Vec2);
                    }
                }
                // Special case for Vec3: if we have exactly three lengths with names "x", "y" and "z", we can create a Vec3
                (3, true) => {
                    if let (Some(_), Some(_), Some(_)) =
                        (map.get(x_ident), map.get(y_ident), map.get(z_ident))
                    {
                        return Ok(Type::Vec3);
                    }
                }
                _ => {}
            }

            Ok(Type::NamedTuple(crate::langtype::NamedTupleType(map)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{langtype::Ty, parser::Rule, tuple::TupleExpression};

    #[test]
    fn unnamed_tuple() {
        use crate::eval::Eval;
        use crate::langtype::Type;

        let input = "(1.0, 2.0m, 3.0)mm";
        let expr = crate::parser::Parser::parse_rule_or_panic::<TupleExpression>(
            Rule::tuple_expression,
            input,
        );
        let value = expr.eval(None).unwrap();
        assert_eq!(
            value.ty(),
            Type::UnnamedTuple(crate::langtype::UnnamedTupleType(vec![
                Type::Length,
                Type::Length,
                Type::Length
            ]))
        );
    }

    #[test]
    fn test_named_tuple() {
        use crate::eval::Eval;
        use crate::langtype::Type;

        let input = "(a = 1.0, b = 2.0m, c = 3.0°)mm";
        let expr = crate::parser::Parser::parse_rule_or_panic::<TupleExpression>(
            Rule::tuple_expression,
            input,
        );
        let value = expr.eval(None).unwrap();
        assert_eq!(
            value.ty(),
            Type::NamedTuple(crate::langtype::NamedTupleType(
                vec![
                    ("a".into(), Type::Length),
                    ("b".into(), Type::Length),
                    ("c".into(), Type::Angle)
                ]
                .into_iter()
                .collect()
            ))
        );
    }

    #[test]
    fn test_vec2() {
        use crate::eval::Eval;
        use crate::langtype::Type;

        let input = "((x,y) = 1mm)";
        let expr = crate::parser::Parser::parse_rule_or_panic::<TupleExpression>(
            Rule::tuple_expression,
            input,
        );
        let value = expr.eval(None).unwrap();
        assert_eq!(value.ty(), Type::Vec2);
    }

    #[test]
    fn test_vec3() {
        use crate::eval::Eval;
        use crate::langtype::Type;

        let input = "(x = 1mm, (y,z) = 2)mm";
        let expr = crate::parser::Parser::parse_rule_or_panic::<TupleExpression>(
            Rule::tuple_expression,
            input,
        );
        let value = expr.eval(None).unwrap();
        assert_eq!(value.ty(), Type::Vec3);
    }
}

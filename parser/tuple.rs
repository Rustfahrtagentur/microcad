use std::collections::BTreeMap;

use crate::call::CallArgumentList;
use crate::eval::{Context, Eval};
use crate::lang_type::{Ty, Type};
use crate::parser::{Pair, Parse, ParseError, ParseResult};
use crate::units::Unit;
use crate::value::{NamedTuple, UnnamedTuple, Value, ValueList, Vec2, Vec3};
use crate::with_pair_ok;

#[derive(Debug, Default, Clone)]
pub struct TupleExpression(CallArgumentList, Option<Unit>);

impl Parse for TupleExpression {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        let mut pairs = pair.into_inner();
        let call_argument_list = CallArgumentList::parse(pairs.next().unwrap())?;
        if call_argument_list.is_empty() {
            return Err(ParseError::EmptyTupleExpression);
        }
        if call_argument_list.contains_positional() && call_argument_list.contains_named() {
            return Err(ParseError::MixedTupleArguments);
        }

        with_pair_ok!(
            TupleExpression(
                call_argument_list.value().clone(),
                match pairs.next() {
                    Some(pair) => Some(*Unit::parse(pair)?),
                    None => None,
                },
            ),
            p
        )
    }
}

impl std::fmt::Display for TupleExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0.contains_positional() {
            write!(f, "(")?;
            for (i, expr) in self.0.get_positional().iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", expr)?;
            }
            write!(f, ")")?;
        } else {
            write!(f, "(")?;
            for (i, (ident, expr)) in self.0.get_named().iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{} = {}", ident, expr)?;
            }
            write!(f, ")")?;
        }
        if let Some(unit) = self.1 {
            write!(f, "{}", unit)?;
        }
        Ok(())
    }
}

impl Eval for TupleExpression {
    type Output = crate::value::Value;

    fn eval(&self, context: &mut Context) -> Result<Value, crate::eval::Error> {
        if self.0.contains_positional() {
            // Unnamed tuple
            let mut value_list = ValueList::new();
            for expr in self.0.get_positional() {
                let value = expr.clone().eval(context)?;
                value_list.push(value);
            }
            if let Some(unit) = self.1 {
                value_list.add_unit_to_unitless_types(unit)?;
            }
            Ok(Value::UnnamedTuple(UnnamedTuple::new(value_list)))
        } else {
            // Named tuple
            let mut map = BTreeMap::new();
            for (ident, expr) in self.0.get_named() {
                let mut value = expr.clone().eval(context)?;
                if let Some(unit) = self.1 {
                    value.add_unit_to_unitless_types(unit)?;
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
}

#[cfg(test)]
mod tests {
    use crate::{eval::Context, lang_type::Ty, parser::Rule, tuple::TupleExpression};

    #[test]
    fn unnamed_tuple() {
        use crate::eval::Eval;
        use crate::lang_type::Type;

        let input = "(1.0, 2.0, 3.0)mm";
        let expr = crate::parser::Parser::parse_rule_or_panic::<TupleExpression>(
            Rule::tuple_expression,
            input,
        );
        let mut context = Context::default();
        let value = expr.eval(&mut context).unwrap();
        assert_eq!(
            value.ty(),
            Type::UnnamedTuple(crate::lang_type::UnnamedTupleType(vec![
                Type::Length,
                Type::Length,
                Type::Length
            ]))
        );
    }

    #[test]
    fn test_named_tuple() {
        use crate::eval::Eval;
        use crate::lang_type::Type;

        let input = "(a = 1.0, b = 2.0, c = 3.0)mm";
        let expr = crate::parser::Parser::parse_rule_or_panic::<TupleExpression>(
            Rule::tuple_expression,
            input,
        );
        let mut context = Context::default();

        let value = expr.eval(&mut context).unwrap();
        assert_eq!(
            value.ty(),
            Type::NamedTuple(crate::lang_type::NamedTupleType(
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
        use crate::eval::Eval;
        use crate::lang_type::Type;

        let input = "((x,y) = 1mm)";
        let expr = crate::parser::Parser::parse_rule_or_panic::<TupleExpression>(
            Rule::tuple_expression,
            input,
        );
        let mut context = Context::default();

        let value = expr.eval(&mut context).unwrap();
        assert_eq!(value.ty(), Type::Vec2);
    }

    #[test]
    fn test_vec3() {
        use crate::eval::Eval;
        use crate::lang_type::Type;

        let input = "(x = 1, y = 2, z = 3)mm";
        let expr = crate::parser::Parser::parse_rule_or_panic::<TupleExpression>(
            Rule::tuple_expression,
            input,
        );
        let mut context = Context::default();

        let value = expr.eval(&mut context).unwrap();
        assert_eq!(value.ty(), Type::Vec3);
    }
}

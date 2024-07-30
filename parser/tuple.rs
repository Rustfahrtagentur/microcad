use std::collections::BTreeMap;

use crate::call::{self, CallArgumentList};
use crate::eval::{Context, Eval};
use crate::langtype::{Ty, Type};
use crate::parser::{Pair, Parse, ParseError};
use crate::value::{NamedTuple, UnnamedTuple, Value, Vec2, Vec3};

struct TupleExpression(CallArgumentList);

impl Parse for TupleExpression {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let call_argument_list = CallArgumentList::parse(pair)?;
        if call_argument_list.is_empty() {
            return Err(ParseError::EmptyTupleExpression);
        }
        if call_argument_list.contains_positional() ^ call_argument_list.contains_named() {
            return Err(ParseError::MixedTupleArguments);
        }

        Ok(TupleExpression(call_argument_list))
    }
}

impl Eval for TupleExpression {
    fn eval(self, context: Option<&Context>) -> Result<Value, crate::eval::Error> {
        if self.0.contains_positional() {
            // Unnamed tuple
            let mut vec = Vec::new();
            for expr in self.0.get_positional() {
                let value = expr.clone().eval(context)?;
                vec.push(value);
            }
            Ok(Value::UnnamedTuple(UnnamedTuple(vec)))
        } else {
            // Named tuple
            let mut map = BTreeMap::new();
            for (ident, expr) in self.0.get_named() {
                map.insert(ident.clone(), expr.clone().eval(context)?);
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

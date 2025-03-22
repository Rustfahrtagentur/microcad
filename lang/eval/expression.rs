use crate::eval::*;

impl Eval for Expression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match self {
            Self::Literal(literal) => Literal::eval(literal, context),
            Self::FormatString(format_string) => FormatString::eval(format_string, context),
            /*Self::ListExpression(list_expression) => ListExpression::eval(list_expression, context),
            Self::TupleExpression(tuple_expression) => {
                TupleExpression::eval(tuple_expression, context)
            }*/
            Self::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref: _,
            } => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;
                if lhs.is_invalid() || rhs.is_invalid() {
                    return Ok(Value::None);
                }

                match op.as_str() {
                    "+" => lhs + rhs,
                    "-" => lhs - rhs,
                    "*" => lhs * rhs,
                    "/" => lhs / rhs,
                    "^" => unimplemented!(), // lhs.pow(&rhs),
                    "&" => lhs & rhs,
                    "|" => lhs | rhs,
                    ">" => Ok(Value::Bool(Refer::new(lhs > rhs, SrcRef::merge(lhs, rhs)))),
                    "<" => Ok(Value::Bool(Refer::new(lhs < rhs, SrcRef::merge(lhs, rhs)))),
                    "≤" => Ok(Value::Bool(Refer::new(lhs <= rhs, SrcRef::merge(lhs, rhs)))),
                    "≥" => Ok(Value::Bool(Refer::new(lhs >= rhs, SrcRef::merge(lhs, rhs)))),
                    "~" => todo!("implement near ~="),
                    "=" => Ok(Value::Bool(Refer::new(lhs == rhs, SrcRef::merge(lhs, rhs)))),
                    "!=" => Ok(Value::Bool(Refer::new(lhs != rhs, SrcRef::merge(lhs, rhs)))),
                    _ => unimplemented!("{op:?}"),
                }
                .map_err(EvalError::ValueError)
            }
            Self::UnaryOp {
                op,
                rhs,
                src_ref: _,
            } => {
                let rhs = rhs.eval(context)?;
                match op.as_str() {
                    "-" => -rhs.clone(),
                    _ => unimplemented!(),
                }
                .map_err(EvalError::ValueError)
            }
            Self::ListElementAccess(lhs, rhs, _) => {
                let lhs = lhs.eval(context)?;
                let rhs = rhs.eval(context)?;

                match (lhs, rhs) {
                    (Value::List(list), Value::Integer(index)) => {
                        let index = index.value as usize;
                        if index < list.len() {
                            match list.get(index) {
                                Some(value) => Ok(value.clone()),
                                None => Err(EvalError::ListIndexOutOfBounds {
                                    index,
                                    len: list.len(),
                                }),
                            }
                        } else {
                            context.error(
                                self,
                                EvalError::ListIndexOutOfBounds {
                                    index,
                                    len: list.len(),
                                },
                            )?;
                            Ok(Value::None)
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            Self::Nested(nested) => nested.eval(context),
            expr => todo!("{expr}"),
        }
    }
}

impl Eval for Nested {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let mut nodes = Vec::new();

        for item in self.iter() {
            match item.eval(context)? {
                Value::Node(n) => nodes.push(n),
                Value::None => {
                    if nodes.is_empty() && self.len() == 1 {
                        return Ok(Value::None);
                    } else {
                        context.error(self, EvalError::CannotNestItem(item.clone()))?;
                    }
                }
                value => {
                    if nodes.is_empty() && self.len() == 1 {
                        return Ok(value);
                    } else {
                        context.error(self, EvalError::CannotNestItem(item.clone()))?;
                    }
                }
            }
        }

        if nodes.is_empty() {
            Ok(Value::None)
        } else {
            todo!("Nest nodes is WIP")
            //Ok(Value::Node(crate::objects::nest_nodes(nodes)))
        }
    }
}

impl Eval for NestedItem {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match &self {
            NestedItem::Call(call) => Ok(call.eval(context)?),
            NestedItem::QualifiedName(qualified_name) => Ok(qualified_name.eval(context)?),
            NestedItem::Body(body) => Ok(body.eval(context)?),
        }
    }
}

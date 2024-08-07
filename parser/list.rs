use crate::eval::{Context, Eval};
use crate::expression::{Expression, ExpressionList};
use crate::parser::{Pair, Parse, ParseError};
use crate::units::Unit;
use crate::value::{Value, ValueList};

#[derive(Debug, Default, Clone)]
pub struct ListExpression(ExpressionList, Option<Unit>);

impl ListExpression {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&Expression> {
        self.0.get(index)
    }
}

impl Parse for ListExpression {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();
        Ok(Self(
            ExpressionList::parse(pairs.next().unwrap())?,
            match pairs.next() {
                Some(pair) => Some(Unit::parse(pair)?),
                None => None,
            },
        ))
    }
}

impl std::fmt::Display for ListExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, expr) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", expr)?;
        }
        write!(f, "]")?;
        if let Some(unit) = self.1 {
            write!(f, "{}", unit)?;
        }
        Ok(())
    }
}

impl Eval for ListExpression {
    type Output = crate::value::Value;

    fn eval(&self, context: &mut Context) -> Result<Value, crate::eval::Error> {
        let mut value_list = ValueList::new();
        for expr in self.0.clone() {
            value_list.push(expr.eval(context)?);
        }
        if let Some(unit) = self.1 {
            value_list.add_unit_to_unitless_types(unit)?;
        }

        match value_list.types().common_type() {
            Some(common_type) => Ok(Value::List(crate::value::List(value_list, common_type))),
            None => Err(crate::eval::Error::ListElementsDifferentTypes),
        }
    }
}

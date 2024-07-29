use crate::eval::{Context, Eval};
use crate::expression::{Expression, ExpressionList};
use crate::langtype::Type;
use crate::parser::{Pair, Parse, ParseError};
use crate::value::{self, Value};

#[derive(Default, Clone)]
pub struct ListExpression(ExpressionList);

impl ListExpression {
    pub fn new(expression_list: ExpressionList) -> Self {
        Self(expression_list)
    }

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
        Ok(Self::new(ExpressionList::parse(
            pair.into_inner().next().unwrap(),
        )?))
    }
}

impl Eval for ListExpression {
    fn eval(self, context: Option<&Context>) -> Result<Value, crate::eval::Error> {
        let mut vec = Vec::new();
        let mut types = Vec::new();
        for expr in self.0 {
            let value = expr.eval(context)?;
            use crate::langtype::Ty;
            types.push(value.ty());
            vec.push(value);
        }
        types.dedup();
        let common_type = match types.len() {
            1 => Some(types.first().unwrap().clone()),
            _ => None,
        };

        Ok(Value::List(crate::value::List(vec, common_type)))
    }

    fn eval_type(&self, context: Option<&Context>) -> Result<Type, crate::eval::Error> {
        self.0.common_eval_type(context)
    }
}

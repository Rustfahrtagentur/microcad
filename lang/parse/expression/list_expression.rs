use crate::{eval::*, parse::*, parser::*};

#[derive(Default, Clone, Debug)]
pub struct ListExpression(ExpressionList, Option<Unit>);

impl std::ops::Deref for ListExpression {
    type Target = ExpressionList;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ListExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Parse for ListExpression {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut inner = pair.clone().into_inner();
        Ok(Self(
            ExpressionList::parse(inner.next().unwrap())?,
            match inner.next() {
                Some(pair) => Some(Unit::parse(pair)?),
                None => None,
            },
        ))
    }
}

impl std::fmt::Display for ListExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{}]{}",
            self.0
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            if let Some(unit) = self.1 {
                unit.to_string()
            } else {
                String::new()
            }
        )?;

        Ok(())
    }
}

impl Eval for ListExpression {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Value> {
        let mut value_list = ValueList::new();
        for expr in self.0.clone() {
            value_list.push(expr.eval(context)?);
        }
        if let Some(unit) = self.1 {
            value_list.add_unit_to_unitless_types(unit)?;
        }

        match value_list.types().common_type() {
            Some(common_type) => Ok(Value::List(List(value_list, common_type))),
            None => Err(EvalError::ListElementsDifferentTypes),
        }
    }
}

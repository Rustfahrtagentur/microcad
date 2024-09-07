//! List expression

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// List expression (expression list maybe with common unit)
#[derive(Default, Clone, Debug)]
pub struct ListExpression(ExpressionList, Option<Unit>, SrcRef);

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

impl SrcReferrer for ListExpression {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.2.clone()
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
            pair.into(),
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
            Some(common_type) => Ok(Value::List(Refer::new(
                List::new(value_list, common_type),
                self.src_ref(),
            ))),
            None => Err(EvalError::ListElementsDifferentTypes),
        }
    }
}

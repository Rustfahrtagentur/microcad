use crate::eval::*;

impl Eval for FormatExpression {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        Ok(Value::String(Refer::new(
            format!("{}", self.expression.eval(context)?),
            SrcRef(None),
        )))
    }
}

impl Eval for FormatString {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let mut result = String::new();
        for elem in &self.0 {
            match elem {
                FormatStringInner::String(s) => result += &s.value,
                FormatStringInner::FormatExpression(expr) => match expr.eval(context) {
                    Ok(Value::String(s)) => result += &s,
                    Err(e) => return Err(e),
                    _ => unreachable!("FormatExpression must always evaluate to a string"),
                },
            }
        }
        Ok(Value::String(Refer::new(result, SrcRef::from_vec(&self.0))))
    }
}

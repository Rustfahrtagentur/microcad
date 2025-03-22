use crate::eval::*;

impl Eval for Assignment {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        let value = self.value.eval(context)?;
        context.add_local_value(self.name.id().clone(), value.clone());
        Ok(value)
    }
}

impl Eval for Statement {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match self {
            Self::Use(u) => u.eval(context)?,
            Self::Assignment(a) => a.eval(context)?,
            Self::Expression(e) => e.eval(context)?,
            statement => todo!("{statement}"),
        };

        Ok(Value::None)
    }
}

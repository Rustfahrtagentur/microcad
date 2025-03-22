use crate::eval::*;

impl Eval for QualifiedName {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        context.fetch_value(self)
    }
}

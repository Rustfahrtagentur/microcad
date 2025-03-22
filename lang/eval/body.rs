use crate::eval::*;

impl Eval for Body {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        context.open_scope();
        let result = Body::evaluate_vec(&self.statements, context);
        context.close_scope();
        result
    }
}

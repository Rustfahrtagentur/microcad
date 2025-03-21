use crate::{eval::*, syntax::*, ty::*};

impl CallArgument {
    /// Evaluate argument as boolean value
    pub fn eval_bool(&self, context: &mut EvalContext) -> EvalResult<bool> {
        match self.value.eval(context) {
            Ok(Value::Bool(cond)) => Ok(*cond),
            Ok(value) => {
                context.error(
                    self.src_ref(),
                    Box::new(EvalError::InvalidArgumentType(value.ty().clone())),
                )?;
                unreachable!()
            }
            Err(err) => Err(err),
        }
    }
}

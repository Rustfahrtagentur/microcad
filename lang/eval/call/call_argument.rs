use crate::{diag::*, eval::*, src_ref::*, syntax::*, ty::*, value::*};

impl CallArgument {
    /// Evaluate `CallArgument` and return `CallArgumentValue`
    pub fn eval_value(&self, context: &mut EvalContext) -> EvalResult<CallArgumentValue> {
        Ok(CallArgumentValue::new(
            self.name.clone(),
            self.value.eval(context)?,
            self.src_ref.clone(),
        ))
    }

    /// Evaluate argument as boolean value
    pub fn eval_bool(&self, context: &mut EvalContext) -> EvalResult<bool> {
        match self.value.eval(context) {
            Ok(Value::Bool(cond)) => Ok(*cond),
            Ok(Value::None) => Ok(false),
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

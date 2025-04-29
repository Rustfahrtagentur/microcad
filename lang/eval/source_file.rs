use crate::{eval::*, syntax::*};

impl Eval for SourceFile {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        context.open_namespace(self.id());
        let result = Body::evaluate_vec(&self.body, context);
        log::trace!("Evaluated context:\n{context}");
        result
    }
}

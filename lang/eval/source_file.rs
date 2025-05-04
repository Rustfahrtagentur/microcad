use crate::{eval::*, syntax::*};

impl Eval for SourceFile {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        context.open_source(self.id());
        let result = Body::evaluate_vec(&self.body, context);
        context.close();
        log::trace!("Evaluated context:\n{context}");
        result
    }
}

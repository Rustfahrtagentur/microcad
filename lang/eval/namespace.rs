use crate::{eval::*, syntax::*};

impl Eval for NamespaceDefinition {
    fn eval(&self, context: &mut Context) -> EvalResult<Value> {
        context.scope(
            StackFrame::Namespace(self.id.clone(), SymbolMap::default()),
            |context| self.body.eval(SymbolMap::default(), context),
        )
    }
}

use crate::eval::*;

pub struct Interpreter {
    /// Context
    context: Context,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            context: Context::default(),
        }
    }

    pub fn eval(&mut self, source_file: SourceFile) -> Result<tree::Node> {
        let mut context = self.context.clone();
        context.add_source_file(source_file);

        let node = context.current_source_file().eval(&mut context)?;

        context.info(crate::src_ref::SrcRef(None), "Evaluation complete".into());
        Ok(node)
    }
}

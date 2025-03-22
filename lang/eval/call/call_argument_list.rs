use crate::{eval::*, syntax::*};

impl CallArgumentList {
    /// Get matching arguments from parameter list
    pub fn get_matching_arguments(
        &self,
        _context: &mut EvalContext,
        _parameters: &ParameterList,
    ) -> EvalResult<ArgumentMap> {
        todo!()
    }

    /// return a single argument
    pub fn get_single(&self) -> EvalResult<&CallArgument> {
        if self.len() == 1 {
            if let Some(a) = self.0.first() {
                return Ok(a);
            }
        }

        Err(EvalError::ArgumentCountMismatch {
            args: self.clone(),
            expected: 1,
            found: self.len(),
        })
    }
}

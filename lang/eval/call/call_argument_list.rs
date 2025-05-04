use crate::{eval::*, syntax::*};

impl CallArgumentList {
    /// Get matching arguments from parameter list
    pub fn get_matching_arguments(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> EvalResult<ArgumentMap> {
        let parameter_values = ParameterValueList::from_parameter_list(parameters, context)?;
        CallArgumentValueList::from_call_argument_list(self, context)?
            .get_matching_arguments(&parameter_values)
    }

    /// Get multiplicty of matching arguments from a parameter list
    pub fn get_multi_matching_arguments(
        &self,
        context: &mut Context,
        parameters: &ParameterList,
    ) -> EvalResult<MultiArgumentMap> {
        let parameter_values = ParameterValueList::from_parameter_list(parameters, context)?;
        CallArgumentValueList::from_call_argument_list(self, context)?
            .get_multi_matching_arguments(&parameter_values)
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

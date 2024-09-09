//! Parameter value list evaluation entity

use crate::{eval::*, src_ref::*};

/// List of parameter values
#[derive(Clone, Debug, Default)]
pub struct ParameterValueList {
    /// List of parameter values
    parameters: Vec<ParameterValue>,
    /// access map by name
    by_name: std::collections::HashMap<Id, usize>,
    /// Source code reference
    src_ref: SrcRef,
}

impl ParameterValueList {
    #[cfg(test)]
    pub fn new(parameters: Vec<ParameterValue>) -> Self {
        let mut by_name = std::collections::HashMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            by_name.insert(parameter.name.clone(), i);
        }

        Self {
            by_name,
            src_ref: SrcRef::from_vec(&parameters),
            parameters,
        }
    }

    /// Push parameter value
    pub fn push(&mut self, parameter: ParameterValue) -> std::result::Result<(), EvalError> {
        if self.by_name.contains_key(&parameter.name) {
            return Err(EvalError::DuplicateParameter(parameter.name.clone()));
        }

        self.by_name
            .insert(parameter.name.clone(), self.parameters.len());
        self.parameters.push(parameter);
        Ok(())
    }

    /// fetch parameter value by name
    pub fn get(&self, name: &Id) -> Option<&ParameterValue> {
        self.by_name.get(name).map(|i| &self.parameters[*i])
    }

    /// remove parameter value by name
    pub fn remove(&mut self, name: &Id) {
        if let Some(new_index) = self.by_name.remove(name) {
            self.parameters.remove(new_index);
            for index in &mut self.by_name.values_mut() {
                if *index > new_index {
                    *index -= 1;
                }
            }
        }
    }

    /// Return `true` if empty
    pub fn is_empty(&self) -> bool {
        self.by_name.is_empty()
    }
}

impl SrcReferrer for ParameterValueList {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::ops::Deref for ParameterValueList {
    type Target = Vec<ParameterValue>;

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

use crate::eval::*;

#[derive(Clone, Debug, Default)]
pub struct ParameterValueList {
    parameters: Vec<ParameterValue>,
    by_name: std::collections::HashMap<Id, usize>,
}

impl ParameterValueList {
    pub fn new(parameters: Vec<ParameterValue>) -> Self {
        let mut by_name = std::collections::HashMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            by_name.insert(parameter.name.clone(), i);
        }

        Self {
            parameters,
            by_name,
        }
    }

    pub fn push(&mut self, parameter: ParameterValue) -> std::result::Result<(), EvalError> {
        if self.by_name.contains_key(&parameter.name) {
            return Err(EvalError::DuplicateParameter(parameter.name.clone()));
        }

        self.by_name
            .insert(parameter.name.clone(), self.parameters.len());
        self.parameters.push(parameter);
        Ok(())
    }

    pub fn get(&self, name: &Id) -> Option<&ParameterValue> {
        self.by_name.get(name).map(|i| &self.parameters[*i])
    }

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

    pub fn is_empty(&self) -> bool {
        self.by_name.is_empty()
    }
}

impl std::ops::Deref for ParameterValueList {
    type Target = Vec<ParameterValue>;

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

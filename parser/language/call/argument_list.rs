use super::{Identifier, Value};

#[derive(Clone, Debug)]
pub struct ArgumentMap(std::collections::HashMap<Identifier, Value>);

impl ArgumentMap {
    pub fn new() -> Self {
        Self(std::collections::HashMap::new())
    }
}

impl Default for ArgumentMap {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for ArgumentMap {
    type Target = std::collections::HashMap<Identifier, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ArgumentMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

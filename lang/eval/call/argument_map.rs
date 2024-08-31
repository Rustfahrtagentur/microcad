use crate::eval::*;

#[derive(Clone, Debug)]
pub struct ArgumentMap(std::collections::HashMap<Id, Value>);

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
    type Target = std::collections::HashMap<Id, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ArgumentMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[macro_export]
macro_rules! args {
    ($($name:ident: $ty:ident = $value:expr),*) => {&{
        let mut map = ArgumentMap::new();
        $(map.insert(stringify!($name).into(), microcad_lang::eval::Value::$ty($value));)*
        map
    }};
    () => {

    };
}

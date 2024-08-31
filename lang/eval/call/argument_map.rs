use crate::eval::*;

#[derive(Clone, Debug, Default)]
pub struct ArgumentMap(std::collections::HashMap<Id, Value>);

impl ArgumentMap {
    pub fn new() -> Self {
        Self(std::collections::HashMap::new())
    }

    pub fn get_value<'a, T>(&'a self, name: &str) -> T
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        if let Some(value) = self.0.get(name) {
            value.try_into().expect("cannot convert argument value")
        } else {
            unreachable!()
        }
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

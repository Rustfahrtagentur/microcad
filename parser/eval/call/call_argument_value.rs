use crate::{eval::*, language::*, ord_map::OrdMapValue};

#[derive(Clone, Debug)]
pub struct CallArgumentValue {
    pub name: Option<Identifier>,
    pub value: Value,
}

impl OrdMapValue<Identifier> for CallArgumentValue {
    fn key(&self) -> Option<Identifier> {
        self.name.clone()
    }
}

impl CallArgumentValue {
    pub fn new(name: Option<Identifier>, value: Value) -> Self {
        Self { name, value }
    }
}

#[macro_export]
macro_rules! call_argument_value {
    ($name:ident: $ty:ident = $value:expr) => {
        CallArgumentValue::new(Some(stringify!($name).into()), Value::$ty($value))
    };
    ($ty:ident = $value:expr) => {
        CallArgumentValue::new(None, Value::$ty($value))
    };
    () => {};
}

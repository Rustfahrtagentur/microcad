use crate::{eval::*, language::*};

#[macro_export]
macro_rules! named_tuple {
    ($($name:ident: $ty:ident = $value:expr),*) => {
        NamedTuple::from_vec(vec![$((stringify!($name).into(), Value::$ty($value)),)*])
    };
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamedTuple(std::collections::BTreeMap<Identifier, Value>);

impl std::ops::Deref for NamedTuple {
    type Target = std::collections::BTreeMap<Identifier, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for NamedTuple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<std::collections::BTreeMap<Identifier, Value>> for NamedTuple {
    fn from(value: std::collections::BTreeMap<Identifier, Value>) -> Self {
        Self(value)
    }
}

impl NamedTuple {
    pub fn from_vec(vec: Vec<(Identifier, Value)>) -> Self {
        Self(vec.into_iter().collect())
    }
}

impl std::fmt::Display for NamedTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({items})",
            items = self
                .0
                .iter()
                .map(|(k, v)| format!("{k} => {v}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Ty for NamedTuple {
    fn ty(&self) -> Type {
        Type::NamedTuple(NamedTupleType(
            self.0
                .iter()
                .map(|(name, v)| (name.clone(), v.ty().clone()))
                .collect(),
        ))
    }
}

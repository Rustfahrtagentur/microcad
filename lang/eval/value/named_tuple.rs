use crate::{eval::*, parse::*, r#type::*, src_ref::*};

#[cfg(test)]
#[macro_export]
macro_rules! named_tuple {
    ($($name:ident: $ty:ident = $value:expr),*) => {
        NamedTuple::from_vec(vec![$((stringify!($name).into(), Value::$ty($value)),)*])
    };
}

#[derive(Clone, Debug, PartialEq)]
pub struct NamedTuple(Refer<std::collections::BTreeMap<Identifier, Value>>);

impl NamedTuple {
    pub fn new(map: std::collections::BTreeMap<Identifier, Value>, src_ref: SrcRef) -> Self {
        Self(Refer::new(map, src_ref))
    }
}

impl SrcReferrer for NamedTuple {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

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

use crate::{eval::*, r#type::*};

#[derive(Clone, Debug, PartialEq)]
pub struct List(pub ValueList, pub Type);

impl std::ops::Deref for List {
    type Target = ValueList;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl List {
    pub fn new(ty: Type) -> Self {
        Self(ValueList::new(), ty)
    }
}

impl IntoIterator for List {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{items}]",
            items = self
                .0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Ty for List {
    fn ty(&self) -> Type {
        self.1.clone()
    }
}

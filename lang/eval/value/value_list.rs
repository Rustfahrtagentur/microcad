use crate::{eval::*, parse::*, r#type::*};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ValueList(Vec<Value>);

impl std::ops::Deref for ValueList {
    type Target = Vec<Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ValueList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_unit_to_unitless_types(
        &mut self,
        unit: Unit,
    ) -> std::result::Result<(), ValueError> {
        for value in self.0.iter_mut() {
            value.add_unit_to_unitless_types(unit)?;
        }
        Ok(())
    }

    pub fn types(&self) -> TypeList {
        TypeList::from_types(
            self.0
                .iter()
                .map(|v| v.ty())
                .collect::<Vec<Type>>()
                .into_iter()
                .collect(),
        )
    }
}

impl IntoIterator for ValueList {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::iter::FromIterator<Value> for ValueList {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        ValueList(Vec::from_iter(iter))
    }
}

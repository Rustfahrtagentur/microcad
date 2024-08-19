use super::Type;

pub struct TypeList(Vec<Type>);

impl TypeList {
    pub fn from_types(types: Vec<Type>) -> Self {
        Self(types)
    }

    pub fn common_type(&self) -> Option<Type> {
        let mut common_type = None;
        for ty in &self.0 {
            match common_type {
                None => common_type = Some(ty.clone()),
                Some(ref t) if t == ty => {}
                _ => return None,
            }
        }
        common_type
    }
}

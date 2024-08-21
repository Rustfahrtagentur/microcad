use crate::language::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Map(
    pub std::collections::HashMap<MapKeyValue, Value>,
    pub MapKeyType,
    Type,
);

impl From<Map> for std::collections::HashMap<MapKeyValue, Value> {
    fn from(val: Map) -> Self {
        val.0
    }
}

impl Ty for Map {
    fn ty(&self) -> Type {
        self.2.clone()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{items}]",
            items = self
                .0
                .iter()
                .map(|(k, v)| format!("{k} => {v}"))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

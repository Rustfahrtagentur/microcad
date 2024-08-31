use crate::{map_key_type::*, parser::*, r#type::*, with_pair_ok};

#[derive(Debug, Clone, PartialEq)]
pub struct MapType(MapKeyType, Box<Type>);

impl MapType {
    pub fn from_types(key: MapKeyType, value: Type) -> Self {
        Self(key, Box::new(value))
    }
}

impl Parse for MapType {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut inner = pair.clone().into_inner();
        let key = inner.next().unwrap();
        let value = inner.next().unwrap();

        with_pair_ok!(
            Self::from_types(
                (Type::parse(key)?.value).try_into()?,
                Type::parse(value)?.value,
            ),
            pair
        )
    }
}

impl std::fmt::Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} => {}]", self.0, self.1)
    }
}

#[test]
fn map_type() {
    use crate::parser::{Parser, Rule};

    let ty = Parser::parse_rule_or_panic::<Type>(Rule::r#type, "[int => string]");
    assert_eq!(ty.to_string(), "[int => string]");
    assert_eq!(
        ty,
        Type::Map(MapType::from_types(MapKeyType::Integer, Type::String))
    );
}

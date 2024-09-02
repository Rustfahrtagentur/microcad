use crate::{errors::*, map_key_type::*, parser::*, r#type::*};

#[derive(Debug, Clone, PartialEq)]
pub struct MapType(MapKeyType, Box<Type>);

impl MapType {
    pub fn from_types(key: MapKeyType, value: Type) -> Self {
        Self(key, Box::new(value))
    }
}

impl Parse for MapType {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut inner = pair.clone().into_inner();
        let key = inner.next().unwrap();
        let value = inner.next().unwrap();

        use crate::eval::Ty;

        Ok(Self::from_types(
            (TypeAnnotation::parse(key)?.ty()).try_into()?,
            TypeAnnotation::parse(value)?.ty(),
        ))
    }
}

impl std::fmt::Display for MapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} => {}]", self.0, self.1)
    }
}

#[test]
fn map_type() {
    use crate::eval::Ty;
    use crate::parser::{Parser, Rule};

    let type_annotation =
        Parser::parse_rule_or_panic::<TypeAnnotation>(Rule::r#type, "[int => string]");
    assert_eq!(type_annotation.ty().to_string(), "[int => string]");
    assert_eq!(
        type_annotation.ty(),
        Type::Map(MapType::from_types(MapKeyType::Integer, Type::String))
    );
}

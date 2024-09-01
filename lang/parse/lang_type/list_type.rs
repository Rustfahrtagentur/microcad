use crate::{eval::*, parser::*, r#type::*};

#[derive(Debug, Clone, PartialEq)]
pub struct ListType(Box<Type>);

impl ListType {
    pub fn from_type(t: Type) -> Self {
        Self(Box::new(t))
    }
}

impl Ty for ListType {
    fn ty(&self) -> Type {
        self.0.as_ref().clone()
    }
}

impl Parse for ListType {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut inner = pair.clone().into_inner();

        let pair = inner.next().unwrap();
        match pair.as_rule() {
            Rule::r#type => Ok(Self::from_type(TypeAnnotation::parse(pair.clone())?.ty())),
            _ => unreachable!("Expected type, found {:?}", pair.as_rule()),
        }
    }
}

impl std::fmt::Display for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[test]
fn list_type() {
    use crate::parser::{Parser, Rule};
    let type_annotation = Parser::parse_rule_or_panic::<TypeAnnotation>(Rule::r#type, "[int]");
    assert_eq!(type_annotation.ty().to_string(), "[int]");
    assert_eq!(
        type_annotation.ty(),
        Type::List(ListType::from_type(Type::Integer))
    );
}

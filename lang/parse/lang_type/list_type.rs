//! List type parser entity

use crate::{errors::*, eval::*, parser::*, r#type::*};

#[allow(rustdoc::broken_intra_doc_links)]
/// List type (e.g. '[scalar]')
#[derive(Debug, Clone, PartialEq)]
pub struct ListType(Box<Type>);

impl ListType {
    /// Generate `ListType` from `Type`
    fn new(t: Type) -> Self {
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
            Rule::r#type => Ok(Self::new(TypeAnnotation::parse(pair.clone())?.ty())),
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
        Type::List(ListType::new(Type::Integer))
    );
}

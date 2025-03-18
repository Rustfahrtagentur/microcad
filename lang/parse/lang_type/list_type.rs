// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! List type parser entity

use crate::{parse::*, parser::*, r#type::*};

#[allow(rustdoc::broken_intra_doc_links)]
/// List type (e.g. '[scalar]')
#[derive(Debug, Clone, PartialEq)]
pub struct ListType(Box<Type>);

impl ListType {
    /// Generate `ListType` from `Type`
    pub fn new(t: Type) -> Self {
        Self(Box::new(t))
    }
}

impl crate::ty::Ty for ListType {
    fn ty(&self) -> Type {
        self.0.as_ref().clone()
    }
}

impl Parse for ListType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();

        let pair = inner.next().expect("Expected type");
        match pair.as_rule() {
            Rule::r#type => Ok(Self::new(TypeAnnotation::parse(pair.clone())?.ty())),
            _ => unreachable!("Expected type, found {:?}", pair.as_rule()),
        }
    }
}

impl std::fmt::Display for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[test]
fn list_type() {
    use crate::parser::{Parser, Rule};
    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "[Int]", 0).expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "[Int]");
    assert_eq!(
        type_annotation.ty(),
        Type::List(ListType::new(Type::Integer))
    );
}

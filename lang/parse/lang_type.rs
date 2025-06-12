// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*, ty::*};

impl Parse for ListType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        use crate::ty::Ty;

        let pair = inner.next().expect("Expected type");
        match pair.as_rule() {
            Rule::r#type => Ok(Self::new(TypeAnnotation::parse(pair.clone())?.ty())),
            _ => unreachable!("Expected type, found {:?}", pair.as_rule()),
        }
    }
}

impl Parse for NamedTupleType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        use crate::ty::Ty;
        Parser::ensure_rule(&pair, Rule::named_tuple_type);

        let mut types = std::collections::BTreeMap::new();

        for pair in pair.inner() {
            let mut inner = pair.inner();
            let name = Identifier::parse(inner.next().expect("Identifier expected"))?;
            let ty = TypeAnnotation::parse(inner.next().expect("Type annotation expected"))?.ty();
            if types.contains_key(&name) {
                return Err(ParseError::DuplicatedMapType(name.clone()));
            }
            types.insert(name, ty);
        }

        Ok(Self(types))
    }
}

impl Parse for UnnamedTupleType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let inner = pair.inner();
        let mut types = Vec::new();
        use crate::ty::Ty;
        for pair in inner {
            types.push(TypeAnnotation::parse(pair)?.ty());
        }

        Ok(Self(types))
    }
}

impl Parse for MatrixType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::matrix_type);

        let mut m: Option<usize> = None;
        let mut n: Option<usize> = None;

        for p in pair.inner() {
            match p.as_rule() {
                Rule::int => match m {
                    None => m = Some(p.as_str().parse().expect("Valid integer")),
                    Some(_) => n = Some(p.as_str().parse().expect("Valid integer")),
                },
                _ => unreachable!(),
            }
        }

        let m = m.expect("M");

        Ok(Self {
            rows: m,
            columns: n.unwrap_or(m),
        })
    }
}

#[test]
fn list_type() {
    use crate::parser::{Parser, Rule};
    use crate::ty::Ty;

    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "[Integer]", 0).expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "[Integer]");
    assert_eq!(
        type_annotation.ty(),
        Type::List(ListType::new(Type::Integer))
    );
}

#[test]
fn unnamed_tuple_type() {
    use crate::parser::*;
    use crate::ty::Ty;

    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "(Integer, String)", 0)
            .expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "(Integer, String)");
    assert_eq!(
        type_annotation.ty(),
        Type::UnnamedTuple(UnnamedTupleType(vec![Type::Integer, Type::String]))
    );
}

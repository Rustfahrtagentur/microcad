// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, src_ref::*, ty::*};

impl Parse for TypeAnnotation {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::r#type);
        let inner = pair.inner().next().expect("Expected type");

        let s = match inner.as_rule() {
            Rule::list_type => Self(Refer::new(Type::List(ListType::parse(inner)?), pair.into())),
            Rule::tuple_type => Self(Refer::new(
                Type::Tuple(TupleType::parse(inner)?),
                pair.into(),
            )),
            Rule::named_tuple_type => Self(Refer::new(
                Type::NamedTuple(NamedTupleType::parse(inner)?),
                pair.into(),
            )),
            Rule::matrix_type => Self(Refer::new(
                Type::Matrix(MatrixType::parse(inner)?),
                pair.into(),
            )),
            Rule::qualified_name => match inner.as_str() {
                "Integer" => Self(Refer::new(Type::Integer, pair.into())),
                "Scalar" => Self(Refer::new(
                    Type::Quantity(QuantityType::Scalar),
                    pair.into(),
                )),
                "Length" => Self(Refer::new(
                    Type::Quantity(QuantityType::Length),
                    pair.into(),
                )),
                "Angle" => Self(Refer::new(Type::Quantity(QuantityType::Angle), pair.into())),
                "String" => Self(Refer::new(Type::String, pair.into())),
                "Color" => Self(Refer::new(Type::Color, pair.into())),
                "Bool" => Self(Refer::new(Type::Bool, pair.into())),
                _ => Self(Refer::new(
                    Type::Custom(QualifiedName::parse(inner)?),
                    pair.into(),
                )),
            },
            _ => unreachable!("Expected type, found {:?}", inner.as_rule()),
        };

        Ok(s)
    }
}

#[test]
fn named_tuple_type() {
    use crate::parser::*;
    use crate::ty::Ty;

    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "(x: Integer, y: String)", 0)
            .expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "(x: Integer, y: String)");
    assert_eq!(
        type_annotation.ty(),
        Type::NamedTuple(NamedTupleType(
            vec![("x".into(), Type::Integer), ("y".into(), Type::String)]
                .into_iter()
                .collect()
        ))
    );
}

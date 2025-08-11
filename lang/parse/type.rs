// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, src_ref::*, ty::*};

impl Parse for Type {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::r#type);
        let inner = pair.inner().next().expect("Expected type");

        match inner.as_rule() {
            Rule::array_type => Ok(Type::Array(Box::new(Type::parse(
                inner.inner().next().expect("Type"),
            )?))),
            Rule::tuple_type => Ok(Type::Tuple(TupleType::parse(inner)?.into())),
            Rule::matrix_type => Ok(Type::Matrix(MatrixType::parse(inner)?)),
            Rule::quantity_type => Ok(Type::Quantity(QuantityType::parse(inner)?)),
            Rule::base_type => match inner.as_str() {
                // Builtin types.
                "Integer" => Ok(Type::Integer),
                "Bool" => Ok(Type::Bool),
                "String" => Ok(Type::String),
                _ => Err(ParseError::UnknownType(inner.to_string())),
            },
            _ => Err(ParseError::UnknownType(inner.to_string())),
        }
    }
}

impl Parse for QuantityType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::quantity_type);
        Ok(match pair.as_str() {
            "Scalar" => QuantityType::Scalar,
            "Length" => QuantityType::Length,
            "Area" => QuantityType::Area,
            "Angle" => QuantityType::Angle,
            "Volume" => QuantityType::Volume,
            "Weight" => QuantityType::Weight,
            "Density" => QuantityType::Density,
            _ => unreachable!("Expected type, found {:?}", pair.as_str()),
        })
    }
}

impl Parse for TypeAnnotation {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self(Refer::new(Type::parse(pair.clone())?, pair.into())))
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
        Type::Tuple(
            TupleType {
                named: [("x", Type::Integer), ("y", Type::String)]
                    .into_iter()
                    .map(|(id, ty)| (id.into(), ty))
                    .collect(),
                ..Default::default()
            }
            .into()
        )
    );
}

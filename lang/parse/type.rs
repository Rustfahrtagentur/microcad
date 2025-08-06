// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, src_ref::*, ty::*};

impl Parse for Type {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::r#type);
        let inner = pair.inner().next().expect("Expected type");

        Ok(match inner.as_rule() {
            Rule::array_type => {
                Type::Array(Box::new(Type::parse(inner.inner().next().expect("Type"))?))
            }
            Rule::tuple_type => Type::Tuple(TupleType::parse(inner)?.into()),
            Rule::matrix_type => Type::Matrix(MatrixType::parse(inner)?),
            Rule::qualified_name => match inner.as_str() {
                // Builtin types.
                "Integer" => Type::Integer,
                "Bool" => Type::Bool,
                "Scalar" => Type::scalar(),
                "Length" => Type::length(),
                "Area" => Type::Quantity(QuantityType::Area),
                "Angle" => Type::Quantity(QuantityType::Angle),
                "Volume" => Type::Quantity(QuantityType::Volume),
                "Weight" => Type::Quantity(QuantityType::Weight),
                "Density" => Type::Quantity(QuantityType::Density),
                "String" => Type::String,
                "Color" => Type::Tuple(TupleType::new_color().into()),
                "Vec2" => Type::Tuple(TupleType::new_vec2().into()),
                "Vec3" => Type::Tuple(TupleType::new_vec3().into()),
                t => {
                    log::warn!("found unknown builtin type {t}!");
                    Type::Custom(QualifiedName::parse(inner)?)
                }
            },
            _ => unreachable!("Expected type, found {:?}", inner.as_rule()),
        })
    }
}

impl Parse for TypeAnnotation {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::r#type);
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

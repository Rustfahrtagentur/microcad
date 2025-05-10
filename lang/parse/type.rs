// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, src_ref::*, ty::*};

impl Parse for TypeAnnotation {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::r#type);
        let inner = pair.inner().next().expect("Expected type");

        let s = match inner.as_rule() {
            Rule::list_type => Self(Refer::new(Type::List(ListType::parse(inner)?), pair.into())),
            Rule::map_type => Self(Refer::new(Type::Map(MapType::parse(inner)?), pair.into())),
            Rule::unnamed_tuple_type => Self(Refer::new(
                Type::UnnamedTuple(UnnamedTupleType::parse(inner)?),
                pair.into(),
            )),
            Rule::named_tuple_type => Self(Refer::new(
                Type::NamedTuple(NamedTupleType::parse(inner)?),
                pair.into(),
            )),
            Rule::identifier => Self::from_str(inner.as_str(), pair.src_ref()),
            _ => unreachable!("Expected type, found {:?}", inner.as_rule()),
        };

        Ok(s)
    }
}

impl TypeAnnotation {
    /// Create `TypeAnnotation` from type name.
    pub fn from_str(type_name: &str, src_ref: SrcRef) -> TypeAnnotation {
        Self(Refer::new(Type::from_str(type_name), src_ref))
    }
}

impl Type {
    /// Create a type from  type name string.
    ///
    /// Returns a valid type or `Type::Unknown`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(type_name: &str) -> Self {
        match type_name {
            "Int" => Type::Integer,
            "Scalar" => Type::Scalar,
            "String" => Type::String,
            "Color" => Type::Color,
            "Length" => Type::Length,
            "Angle" => Type::Angle,
            "Vec2" => Type::Vec2,
            "Vec3" => Type::Vec3,
            "Bool" => Type::Bool,
            unknown => Type::Unknown(unknown.to_string()),
        }
    }
}

#[test]
fn named_tuple_type() {
    use crate::parser::*;
    use crate::ty::Ty;

    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "(x: Int, y: String)", 0)
            .expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "(x: Int, y: String)");
    assert_eq!(
        type_annotation.ty(),
        Type::NamedTuple(NamedTupleType(
            vec![("x".into(), Type::Integer), ("y".into(), Type::String)]
                .into_iter()
                .collect()
        ))
    );
}

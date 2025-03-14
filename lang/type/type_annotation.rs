// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Basic Types

use crate::{eval::*, parse::*, parser::*, src_ref::*};

use super::Type;

/// Type within source code
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAnnotation(pub Refer<Type>);

impl SrcReferrer for TypeAnnotation {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Ty for TypeAnnotation {
    fn ty(&self) -> Type {
        self.0.value.clone()
    }
}

impl From<Type> for TypeAnnotation {
    fn from(value: Type) -> Self {
        TypeAnnotation(Refer::none(value))
    }
}

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
            Rule::qualified_name => match inner.as_str() {
                "Int" => Self(Refer::new(Type::Integer, pair.into())),
                "Scalar" => Self(Refer::new(Type::Scalar, pair.into())),
                "String" => Self(Refer::new(Type::String, pair.into())),
                "Color" => Self(Refer::new(Type::Color, pair.into())),
                "Length" => Self(Refer::new(Type::Length, pair.into())),
                "Angle" => Self(Refer::new(Type::Angle, pair.into())),
                "Vec2" => Self(Refer::new(Type::Vec2, pair.into())),
                "Vec3" => Self(Refer::new(Type::Vec3, pair.into())),
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

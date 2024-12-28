// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unnamed record parser entity

use crate::{parse::*, parser::*, r#type::*};

/// Unnamed record type (e.g. `(scalar,string)`
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnnamedRecordType(pub Vec<Type>);

impl Parse for UnnamedRecordType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let inner = pair.inner();
        let mut types = Vec::new();
        use crate::eval::Ty;
        for pair in inner {
            types.push(TypeAnnotation::parse(pair)?.ty());
        }

        Ok(Self(types))
    }
}

impl std::fmt::Display for UnnamedRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, t) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", t)?;
        }
        write!(f, ")")
    }
}

#[test]
fn unnamed_record_type() {
    use crate::eval::Ty;
    use crate::parser::*;

    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "(Int, String)", 0).expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "(Int, String)");
    assert_eq!(
        type_annotation.ty(),
        Type::UnnamedRecord(UnnamedRecordType(vec![Type::Integer, Type::String]))
    );
}

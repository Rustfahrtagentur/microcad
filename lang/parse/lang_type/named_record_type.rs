// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named record type parser entity

use crate::{parse::*, parser::*, r#type::*};

/// Named record (e.g. `(n: scalar, m: string)`)
#[derive(Debug, Clone, Default, PartialEq)]
pub struct NamedRecordType(pub std::collections::BTreeMap<Identifier, Type>);

impl Parse for NamedRecordType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::named_record_type);

        let mut types = std::collections::BTreeMap::new();

        use crate::eval::Ty;

        for pair in pair.inner() {
            let mut inner = pair.inner();
            let name = Identifier::parse(inner.next().expect("Identifier expected"))?;
            let ty = TypeAnnotation::parse(inner.next().expect("Type annotation expected"))?.ty();
            if types.contains_key(&name) {
                return Err(ParseError::DuplicatedMapField(name.clone()));
            }
            types.insert(name, ty);
        }

        Ok(Self(types))
    }
}

impl std::fmt::Display for NamedRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, (identifier, ty)) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", identifier, ty)?;
        }
        write!(f, ")")
    }
}

#[test]
fn named_record_type() {
    use crate::eval::Ty;
    use crate::parser::*;

    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "(x: Int, y: String)", 0)
            .expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "(x: Int, y: String)");
    assert_eq!(
        type_annotation.ty(),
        Type::NamedRecord(NamedRecordType(
            vec![
                (Identifier::from("x"), Type::Integer),
                (Identifier::from("y"), Type::String)
            ]
            .into_iter()
            .collect()
        ))
    );
}

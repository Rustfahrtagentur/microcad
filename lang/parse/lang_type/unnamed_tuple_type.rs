//! Unnamed tuple parser entity

use crate::{errors::*, parser::*, r#type::*};

/// Unnamed tuple type (e.g. `(scalar,string)`
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnnamedTupleType(pub Vec<Type>);

impl Parse for UnnamedTupleType {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let inner = pair.clone().into_inner();
        let mut types = Vec::new();
        use crate::eval::Ty;
        for pair in inner {
            types.push(TypeAnnotation::parse(pair)?.ty());
        }

        Ok(Self(types))
    }
}

impl std::fmt::Display for UnnamedTupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
fn unnamed_tuple_type() {
    use crate::eval::Ty;
    use crate::parser::*;

    let type_annotation =
        Parser::parse_rule_or_panic::<TypeAnnotation>(Rule::r#type, "(int, string)");
    assert_eq!(type_annotation.ty().to_string(), "(int, string)");
    assert_eq!(
        type_annotation.ty(),
        Type::UnnamedTuple(UnnamedTupleType(vec![Type::Integer, Type::String]))
    );
}

use crate::{parser::*, r#type::*, with_pair_ok};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct UnnamedTupleType(pub Vec<Type>);

impl Parse for UnnamedTupleType {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let inner = pair.clone().into_inner();
        let mut types = Vec::new();
        for pair in inner {
            types.push(Type::parse(pair)?.value);
        }

        with_pair_ok!(Self(types), pair)
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
    use crate::parser::{Parser, Rule};

    let ty = Parser::parse_rule_or_panic::<Type>(Rule::r#type, "(int, string)");
    assert_eq!(ty.to_string(), "(int, string)");
    assert_eq!(
        ty,
        Type::UnnamedTuple(UnnamedTupleType(vec![Type::Integer, Type::String]))
    );
}

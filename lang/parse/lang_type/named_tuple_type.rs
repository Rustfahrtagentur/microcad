use crate::{errors::*, parse::*, parser::*, r#type::*, with_pair_ok};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NamedTupleType(pub std::collections::BTreeMap<Identifier, Type>);

impl Parse for NamedTupleType {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::named_tuple_type);

        let mut types = std::collections::BTreeMap::new();
        for pair in pair.clone().into_inner() {
            let mut inner = pair.into_inner();
            let name = Identifier::parse(inner.next().unwrap())?.value;
            let ty = Type::parse(inner.next().unwrap())?.value;
            if types.contains_key(&name) {
                return Err(TypeError::DuplicatedMapField(name.clone()).into());
            }
            types.insert(name, ty);
        }

        with_pair_ok!(Self(types), pair)
    }
}

impl std::fmt::Display for NamedTupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
fn named_tuple_type() {
    use crate::parser::{Parser, Rule};

    let ty = Parser::parse_rule_or_panic::<Type>(Rule::r#type, "(x: int, y: string)");
    assert_eq!(ty.to_string(), "(x: int, y: string)");
    assert_eq!(
        ty,
        Type::NamedTuple(NamedTupleType(
            vec![
                (Identifier::from("x"), Type::Integer),
                (Identifier::from("y"), Type::String)
            ]
            .into_iter()
            .collect()
        ))
    );
}

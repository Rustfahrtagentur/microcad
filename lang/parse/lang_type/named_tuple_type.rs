use crate::{errors::*, parse::*, parser::*, r#type::*};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NamedTupleType(pub std::collections::BTreeMap<Identifier, Type>);

impl Parse for NamedTupleType {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::named_tuple_type);

        let mut types = std::collections::BTreeMap::new();

        use crate::eval::Ty;

        for pair in pair.clone().into_inner() {
            let mut inner = pair.into_inner();
            let name = Identifier::parse(inner.next().unwrap())?;
            let ty = TypeAnnotation::parse(inner.next().unwrap())?.ty();
            if types.contains_key(&name) {
                return Err(TypeError::DuplicatedMapField(name.clone()).into());
            }
            types.insert(name, ty);
        }

        Ok(Self(types))
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
    use crate::eval::Ty;
    use crate::parser::*;

    let type_annotation =
        Parser::parse_rule_or_panic::<TypeAnnotation>(Rule::r#type, "(x: int, y: string)");
    assert_eq!(type_annotation.ty().to_string(), "(x: int, y: string)");
    assert_eq!(
        type_annotation.ty(),
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

use crate::{parse::*, parser::*, syntax::*, r#type::*};

impl Parse for ListType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        use crate::ty::Ty;

        let pair = inner.next().expect("Expected type");
        match pair.as_rule() {
            Rule::r#type => Ok(Self::new(TypeAnnotation::parse(pair.clone())?.ty())),
            _ => unreachable!("Expected type, found {:?}", pair.as_rule()),
        }
    }
}

impl Parse for MapType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();
        let key = inner.next().expect("missing key expression");
        let value = inner.next().expect("missing value expression");

        use crate::ty::Ty;

        Ok(Self::new(
            (TypeAnnotation::parse(key)?.ty()).try_into()?,
            TypeAnnotation::parse(value)?.ty(),
        ))
    }
}

impl Parse for NamedTupleType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        use crate::ty::Ty;
        Parser::ensure_rule(&pair, Rule::named_tuple_type);

        let mut types = std::collections::BTreeMap::new();

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

impl Parse for UnnamedTupleType {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let inner = pair.inner();
        let mut types = Vec::new();
        use crate::ty::Ty;
        for pair in inner {
            types.push(TypeAnnotation::parse(pair)?.ty());
        }

        Ok(Self(types))
    }
}

impl TryFrom<Type> for MapKeyType {
    type Error = ParseError;

    fn try_from(t: Type) -> Result<Self, Self::Error> {
        match t {
            Type::Integer => Ok(Self::Integer),
            Type::Bool => Ok(Self::Bool),
            Type::String => Ok(Self::String),
            _ => Err(ParseError::InvalidMapKeyType(t.to_string())),
        }
    }
}

#[test]
fn list_type() {
    use crate::parser::{Parser, Rule};
    use crate::ty::Ty;
    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "[Int]", 0).expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "[Int]");
    assert_eq!(
        type_annotation.ty(),
        Type::List(ListType::new(Type::Integer))
    );
}

#[test]
fn unnamed_tuple_type() {
    use crate::parser::*;
    use crate::ty::Ty;

    let type_annotation =
        Parser::parse_rule::<TypeAnnotation>(Rule::r#type, "(Int, String)", 0).expect("test error");
    assert_eq!(type_annotation.ty().to_string(), "(Int, String)");
    assert_eq!(
        type_annotation.ty(),
        Type::UnnamedTuple(UnnamedTupleType(vec![Type::Integer, Type::String]))
    );
}

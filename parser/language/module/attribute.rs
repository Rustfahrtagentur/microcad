use crate::{language::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: QualifiedName,
    pub arguments: Option<CallArgumentList>,
}

impl Parse for Attribute {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut inner = pair.clone().into_inner();
        let name = QualifiedName::parse(inner.next().unwrap())?.value().clone();
        match inner.next() {
            Some(pair) => with_pair_ok!(
                Attribute {
                    name,
                    arguments: Some(CallArgumentList::parse(pair.clone())?.value().clone()),
                },
                pair
            ),
            _ => with_pair_ok!(
                Attribute {
                    name,
                    arguments: None,
                },
                pair
            ),
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.arguments {
            Some(arguments) => write!(f, "{}({:?})", self.name, arguments),
            None => write!(f, "{}", self.name),
        }
    }
}

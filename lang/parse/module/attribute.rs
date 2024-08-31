use crate::{parse::*, parser::*};

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: QualifiedName,
    pub arguments: Option<CallArgumentList>,
}

impl Parse for Attribute {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut inner = pair.clone().into_inner();
        let name = QualifiedName::parse(inner.next().unwrap())?;
        match inner.next() {
            Some(pair) => Ok(Attribute {
                name,
                arguments: Some(CallArgumentList::parse(pair.clone())?),
            }),
            _ => Ok(Attribute {
                name,
                arguments: None,
            }),
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

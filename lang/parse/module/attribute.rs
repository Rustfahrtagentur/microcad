use crate::{parse::*, parser::*, src_ref::*};

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: QualifiedName,
    pub arguments: Option<CallArgumentList>,
    src_ref: SrcRef,
}

impl SrcReferrer for Attribute {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for Attribute {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut inner = pair.clone().into_inner();
        let name = QualifiedName::parse(inner.next().unwrap())?;
        Ok(Attribute {
            name,
            arguments: match inner.next() {
                Some(pair) => Some(CallArgumentList::parse(pair.clone())?),
                _ => None,
            },
            src_ref: pair.into(),
        })
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

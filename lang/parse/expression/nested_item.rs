use crate::{parse::*, parser::*};

#[derive(Clone, Debug)]
pub enum NestedItem {
    Call(Call),
    QualifiedName(QualifiedName),
    ModuleBody(ModuleBody),
}

impl Parse for NestedItem {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        match pair.clone().as_rule() {
            Rule::call => Ok(NestedItem::Call(Call::parse(pair.clone())?)),
            Rule::qualified_name => Ok(NestedItem::QualifiedName(QualifiedName::parse(
                pair.clone(),
            )?)),
            Rule::module_body => Ok(NestedItem::ModuleBody(ModuleBody::parse(pair.clone())?)),
            rule => unreachable!(
                "NestedItem::parse expected call or qualified name, found {:?}",
                rule
            ),
        }
    }
}

impl std::fmt::Display for NestedItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NestedItem::Call(call) => write!(f, "{}", call),
            NestedItem::QualifiedName(qualified_name) => write!(f, "{}", qualified_name),
            NestedItem::ModuleBody(body) => write!(f, "{}", body),
        }
    }
}

use crate::{language::module::ModuleBody, with_pair_ok};

use super::{Call, Pair, Parse, ParseResult, QualifiedName, Rule};

#[derive(Clone, Debug)]
pub enum NestedItem {
    Call(Call),
    QualifiedName(QualifiedName),
    ModuleBody(ModuleBody),
}

impl Parse for NestedItem {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        match pair.clone().as_rule() {
            Rule::call => with_pair_ok!(
                NestedItem::Call(Call::parse(pair.clone())?.value().clone()),
                pair
            ),
            Rule::qualified_name => {
                with_pair_ok!(
                    NestedItem::QualifiedName(QualifiedName::parse(pair.clone())?.value().clone()),
                    pair
                )
            }
            Rule::module_body => {
                with_pair_ok!(
                    NestedItem::ModuleBody(ModuleBody::parse(pair.clone())?.value().clone()),
                    pair
                )
            }
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

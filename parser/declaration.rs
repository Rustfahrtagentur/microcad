use crate::parser::*;
use crate::{
    expression::Expression,
    identifier::{Identifier, IdentifierList},
};

struct VariableDeclaration {
    names: IdentifierList,
    value: Option<Expression>,
    specified_type: Option<Identifier>,
}

impl Parse for VariableDeclaration {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut names = IdentifierList::default();
        let mut value = None;
        let mut specified_type = None;

        let rule_single_or_multi = pair.as_rule();

        for pair in pair.into_inner() {
            match (rule_single_or_multi, pair.as_rule()) {
                (Rule::variable_single_declaration, Rule::identifier) => {
                    names.push(Identifier::parse(pair)?)?;
                }
                (Rule::variable_multi_declaration, Rule::identifier_list) => {
                    names.extend(IdentifierList::parse(pair)?)?;
                }
                (_, Rule::expression) => value = Some(Expression::parse(pair)?),
                (_, Rule::type_annotation) => specified_type = Some(Identifier::parse(pair)?),
                _ => return Err(ParseError::UnexpectedToken),
            }
        }

        if names.is_empty() {
            Err(ParseError::ExpectedIdentifier)
        } else {
            Ok(Self {
                names,
                value,
                specified_type,
            })
        }
    }
}

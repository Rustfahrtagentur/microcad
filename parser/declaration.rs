use crate::identifier::IdentifierListError;
use crate::lang_type::Type;
use crate::parser::*;
use crate::{expression::Expression, identifier::Identifier};

use std::collections::HashMap;

pub struct VariableDeclaration {
    name: Identifier,
    _default_value: Option<Expression>,
    _specified_type: Option<Type>,
}

impl Parse for VariableDeclaration {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut name = Identifier::default();
        let mut default_value = None;
        let mut specified_type = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::expression => {
                    default_value = Some(Expression::parse(pair)?);
                }
                Rule::r#type => {
                    specified_type = Some(Type::parse(pair)?);
                }
                pair => {
                    unreachable!("Unexpected token in variable declaration: {:?}", pair);
                }
            }
        }

        Ok(Self {
            name,
            _default_value: default_value,
            _specified_type: specified_type,
        })
    }
}

#[derive(Default)]
pub struct VariableDeclarationList {
    decls: Vec<VariableDeclaration>,
    map: HashMap<Identifier, usize>,
}

impl VariableDeclarationList {
    fn push(&mut self, declaration: VariableDeclaration) -> Result<(), IdentifierListError> {
        let name = declaration.name.clone();
        if self.map.contains_key(&name) {
            return Err(IdentifierListError::DuplicateIdentifier(name));
        }
        self.decls.push(declaration);
        self.map.insert(name, self.decls.len() - 1);
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&VariableDeclaration> {
        self.decls.get(index)
    }

    pub fn get_by_name(&self, name: &Identifier) -> Option<&VariableDeclaration> {
        self.map.get(name).and_then(|index| self.get(*index))
    }
}

impl Parse for VariableDeclarationList {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut l = Self::default();

        for pair in pair.into_inner() {
            l.push(VariableDeclaration::parse(pair)?)?;
        }

        Ok(l)
    }
}

pub struct _FunctionSignature(VariableDeclarationList, Type);

#[cfg(test)]
mod tests {

    use super::*;
    use crate::eval::Context;
    use crate::parser::Parser;

    #[test]
    fn variable_declaration() {
        use crate::eval::Eval;
        let decl =
            Parser::parse_rule_or_panic::<VariableDeclaration>(Rule::variable_declaration, "a = 1");

        let context = Context::default();

        assert_eq!(decl.name, Identifier::from("a"));
        assert_eq!(
            decl._default_value
                .unwrap()
                .eval(Some(&context))
                .unwrap()
                .to_string(),
            "1"
        );
        assert!(decl._specified_type.is_none());
    }

    #[test]
    fn variable_declaration_list() {
        let decls = Parser::parse_rule_or_panic::<VariableDeclarationList>(
            Rule::variable_declaration_list,
            "a = 1, b: length = 2mm, c = 2",
        );

        assert_eq!(decls.get(0).unwrap().name, Identifier::from("a"));
        assert_eq!(decls.get(1).unwrap().name, Identifier::from("b"));
        assert_eq!(decls.get(2).unwrap().name, Identifier::from("c"));
        use std::str::FromStr;

        assert_eq!(
            decls
                .get_by_name(&Identifier::from_str("a").unwrap())
                .unwrap()
                .name,
            Identifier::from("a")
        );
    }
}

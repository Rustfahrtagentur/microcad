use crate::identifier::IdentifierListError;
use crate::lang_type::Type;
use crate::parser::*;
use crate::{
    expression::Expression,
    identifier::{Identifier, IdentifierList},
};

use std::collections::HashMap;

pub struct VariableSingleDeclaration {
    name: Identifier,
    _default_value: Option<Expression>,
    _specified_type: Option<Type>,
}

pub struct VariableMultiDeclaration {
    names: IdentifierList,
    default_value: Option<Expression>,
    specified_type: Option<Type>,
}

impl From<VariableMultiDeclaration> for IdentifierList {
    fn from(value: VariableMultiDeclaration) -> Self {
        value.names
    }
}

impl VariableMultiDeclaration {
    pub fn fetch_single_declarations(&self) -> Vec<VariableSingleDeclaration> {
        self.names
            .iter()
            .map(|name| VariableSingleDeclaration {
                name: name.clone(),
                _default_value: self.default_value.clone(),
                _specified_type: self.specified_type.clone(),
            })
            .collect()
    }
}

pub enum VariableDeclaration {
    Single(VariableSingleDeclaration),
    Multi(VariableMultiDeclaration),
}

impl Parse for VariableDeclaration {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut names = IdentifierList::default();
        let mut default_value = None;
        let mut specified_type = None;

        let rule_single_or_multi = pair.as_rule();
        assert!(
            rule_single_or_multi == Rule::variable_single_declaration
                || rule_single_or_multi == Rule::variable_multi_declaration
        );

        for pair in pair.into_inner() {
            match (rule_single_or_multi, pair.as_rule()) {
                (Rule::variable_single_declaration, Rule::identifier) => {
                    names.push(Identifier::parse(pair)?)?;
                }
                (Rule::variable_multi_declaration, Rule::identifier_list) => {
                    names.extend(IdentifierList::parse(pair)?)?;
                }
                (_, Rule::expression) => default_value = Some(Expression::parse(pair)?),
                (_, Rule::type_annotation) => specified_type = Some(Type::parse(pair)?),
                _ => {
                    println!("{:?}", pair.as_rule());
                    return Err(ParseError::UnexpectedToken);
                }
            }
        }

        match names.len() {
            0 => Err(ParseError::ExpectedIdentifier),
            1 => Ok(Self::Single(VariableSingleDeclaration {
                name: names.get(0).unwrap().clone(),
                _default_value: default_value,
                _specified_type: specified_type,
            })),
            _ => Ok(Self::Multi(VariableMultiDeclaration {
                names,
                default_value,
                specified_type,
            })),
        }
    }
}

#[derive(Default)]
pub struct VariableDeclarationList {
    decls: Vec<VariableSingleDeclaration>,
    map: HashMap<Identifier, usize>,
}

impl VariableDeclarationList {
    fn push(&mut self, declaration: VariableDeclaration) -> Result<(), IdentifierListError> {
        match declaration {
            VariableDeclaration::Single(decl) => {
                let name = decl.name.clone();
                if self.map.contains_key(&name) {
                    return Err(IdentifierListError::DuplicateIdentifier(name));
                }
                self.decls.push(decl);
                self.map.insert(name, self.decls.len() - 1);
            }
            VariableDeclaration::Multi(decl) => {
                for name in decl.names.iter() {
                    if self.map.contains_key(name) {
                        return Err(IdentifierListError::DuplicateIdentifier(name.clone()));
                    }
                }

                let single_decl = decl.fetch_single_declarations();
                for decl in single_decl {
                    let name = decl.name.clone();
                    self.decls.push(decl);
                    self.map.insert(name.clone(), self.decls.len());
                }
            }
        }

        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&VariableSingleDeclaration> {
        self.decls.get(index)
    }

    pub fn get_by_name(&self, name: &Identifier) -> Option<&VariableSingleDeclaration> {
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
    fn variable_single_declaration() {
        let decl = Parser::parse_rule_or_panic::<VariableDeclaration>(
            Rule::variable_single_declaration,
            "a = 1",
        );

        let context = Context::default();
        use crate::eval::Eval;

        match decl {
            VariableDeclaration::Single(decl) => {
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
            _ => panic!("Expected single declaration"),
        }
    }

    #[test]
    fn variable_multi_declaration() {
        let decl = Parser::parse_rule_or_panic::<VariableDeclaration>(
            Rule::variable_multi_declaration,
            "(a, b) = 1",
        );

        let context = Context::default();
        use crate::eval::Eval;

        match decl {
            VariableDeclaration::Multi(decl) => {
                assert_eq!(decl.names.len(), 2);
                assert_eq!(decl.names.get(0).unwrap(), &Identifier::from("a"));
                assert_eq!(decl.names.get(1).unwrap(), &Identifier::from("b"));
                assert_eq!(
                    decl.default_value
                        .unwrap()
                        .eval(Some(&context))
                        .unwrap()
                        .to_string(),
                    "1"
                );
                assert!(decl.specified_type.is_none());
            }
            _ => panic!("Expected multi declaration"),
        }
    }

    #[test]
    fn variable_declaration_list() {
        let decls = Parser::parse_rule_or_panic::<VariableDeclarationList>(
            Rule::variable_declaration_list,
            "a = 1, (b, c) = 2",
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

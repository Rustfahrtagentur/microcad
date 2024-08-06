use crate::eval::{Context, Eval};
use crate::identifier::IdentifierListError;
use crate::lang_type::{Ty, Type};
use crate::parser::*;
use crate::value::Value;
use crate::{expression::Expression, identifier::Identifier};

use std::collections::HashMap;

#[derive(Clone)]
pub struct VariableDeclaration {
    name: Identifier,
    default_value: Option<Expression>,
    specified_type: Option<Type>,
}

impl VariableDeclaration {
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn default_value(&self) -> Option<&Expression> {
        self.default_value.as_ref()
    }

    pub fn specified_type(&self) -> Option<&Type> {
        self.specified_type.as_ref()
    }
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
            default_value,
            specified_type,
        })
    }
}

/*
impl Eval for VariableDeclaration {
    fn eval(self, context: Option<&Context>) -> Result<Value, crate::eval::Error> {
        match (self.default_value, self.specified_type) {
            (Some(value_expr), Some(specified_type)) => {
            }
            (Some(value_expr), None) => {
            }
            (None, Some(specified_type)) => {
            }
        if let Some(value_expr) = self.default_value {
            let value = value_expr.eval(context)?;
            if let Some(specified_type) = &self.specified_type {
                if value.ty() == *specified_type {
                    return Ok(value);
                } else {
                    return Err(crate::eval::Error::TypeMismatch {
                        expected: specified_type.clone(),
                        found: value.ty(),
                    });
                }
            } else {
                Ok(value)
            }
        } else {
            Ok(Value::None)
        }
    }

    fn eval_type(&self, _: Option<&Context>) -> Result<Type, crate::eval::Error> {
        if let Some(specified_type) = &self.specified_type {
            Ok(specified_type.clone())
        } else {
            Ok(Type::Angle) // TODO this is a placeholder
        }
    }
}
*/
#[derive(Default, Clone)]
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
            decl.default_value
                .unwrap()
                .eval(Some(&context))
                .unwrap()
                .to_string(),
            "1"
        );
        assert!(decl.specified_type.is_none());
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

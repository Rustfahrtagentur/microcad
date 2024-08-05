use crate::expression::Expression;
use crate::identifier::Identifier;
use crate::parser::*;

use crate::declaration::{VariableDeclaration, VariableDeclarationList};
use crate::lang_type::Type;
use crate::module::UseStatement;

#[derive(Clone)]
pub struct FunctionSignature(VariableDeclarationList, Type);

impl FunctionSignature {
    pub fn arguments(&self) -> &VariableDeclarationList {
        &self.0
    }

    pub fn return_type(&self) -> &Type {
        &self.1
    }
}

impl Parse for FunctionSignature {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();
        Ok(Self(
            VariableDeclarationList::parse(pairs.next().unwrap())?,
            Type::parse(pairs.next().unwrap())?,
        ))
    }
}

#[derive(Clone)]
pub enum FunctionStatement {
    VariableDeclaration(VariableDeclaration),
    Use(UseStatement),
    FunctionDefinition(FunctionDefinition),
    Return(Box<Expression>),
    If {
        condition: Expression,
        if_body: Vec<FunctionStatement>,
        else_body: Vec<FunctionStatement>,
    },
}

impl FunctionStatement {
    pub fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();
        let first = pairs.next().unwrap();
        match first.as_rule() {
            Rule::variable_declaration => Ok(Self::VariableDeclaration(
                VariableDeclaration::parse(first)?,
            )),
            Rule::use_statement => Ok(Self::Use(UseStatement::parse(first)?)),
            Rule::function_definition => {
                Ok(Self::FunctionDefinition(FunctionDefinition::parse(first)?))
            }
            Rule::function_return_statement => {
                Ok(Self::Return(Box::new(Expression::parse(first)?)))
            }
            Rule::function_if_statement => {
                let mut pairs = first.into_inner();
                let condition = Expression::parse(pairs.next().unwrap())?;
                let if_body =
                    Parser::vec(pairs.next().unwrap().into_inner(), FunctionStatement::parse)?;

                match pairs.next() {
                    None => Ok(Self::If {
                        condition,
                        if_body,
                        else_body: Vec::new(),
                    }),
                    Some(pair) => {
                        let else_body = Parser::vec(pair.into_inner(), FunctionStatement::parse)?;
                        Ok(Self::If {
                            condition,
                            if_body,
                            else_body,
                        })
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub signature: FunctionSignature,
    pub body: Vec<FunctionStatement>,
}

impl Parse for FunctionDefinition {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();
        let name = Identifier::parse(pairs.next().unwrap())?;
        let signature = FunctionSignature::parse(pairs.next().unwrap())?;
        let body = Parser::vec(pairs.next().unwrap().into_inner(), FunctionStatement::parse)?;
        Ok(Self {
            name,
            signature,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{declaration::VariableDeclarationList, eval::Symbol};

    #[test]
    fn function_declaration() {
        use crate::declaration::VariableDeclaration;
        use crate::expression::Expression;
        use crate::function::{FunctionDefinition, FunctionSignature, FunctionStatement};
        use crate::identifier::Identifier;
        use crate::lang_type::{Type, TypeList};
        use crate::parser::Parser;
        use crate::parser::Rule;

        let input = "function test(a: scalar, b: scalar) -> scalar {
            c = 1.0;
            return a + b + c;
        }";

        let function_decl =
            Parser::parse_rule_or_panic::<FunctionDefinition>(Rule::function_definition, input);

        let mut context = crate::eval::Context::default();
        context.add_symbol(Symbol::Function(function_decl));

        // context.insert(name, symbol)
    }
}

use crate::expression::Expression;
use crate::identifier::Identifier;
use crate::parser::*;

use crate::declaration::{VariableDeclaration, VariableDeclarationList};
use crate::lang_type::Type;
use crate::module::UseStatement;
pub struct FunctionSignature(VariableDeclarationList, Type);

impl Parse for FunctionSignature {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();
        Ok(Self(
            VariableDeclarationList::parse(pairs.next().unwrap())?,
            Type::parse(pairs.next().unwrap())?,
        ))
    }
}

pub enum FunctionStatement {
    VariableDeclaration(VariableDeclaration),
    Use(UseStatement),
    FunctionDeclaration(FunctionDeclaration),
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
            Rule::function_declaration => Ok(Self::FunctionDeclaration(
                FunctionDeclaration::parse(first)?,
            )),
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

pub struct FunctionDeclaration {
    pub name: Identifier,
    pub signature: FunctionSignature,
    pub body: Vec<FunctionStatement>,
}

impl Parse for FunctionDeclaration {
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

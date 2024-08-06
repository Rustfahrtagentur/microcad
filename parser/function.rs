use crate::expression::Expression;
use crate::identifier::Identifier;
use crate::parser::*;

use crate::lang_type::Type;
use crate::module::UseStatement;

#[derive(Clone)]
pub struct DefinitionParameter {
    #[allow(dead_code)]
    name: Identifier,
    #[allow(dead_code)]
    specified_type: Option<Type>,
    #[allow(dead_code)]
    value: Option<Expression>,
}

impl Parse for DefinitionParameter {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut value = None;

        println!("DefinitionParameter::parse: {:?}", pair);

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::r#type => {
                    specified_type = Some(Type::parse(pair)?);
                }
                Rule::expression => {
                    value = Some(Expression::parse(pair)?);
                }
                rule => {
                    unreachable!(
                        "Unexpected token in definition parameter: {:?} {:?}",
                        rule,
                        pair.as_span().as_str()
                    );
                }
            }
        }

        if specified_type.is_none() && value.is_none() {
            return Err(ParseError::DefinitionParameterMissingTypeOrValue(
                name.clone(),
            ));
        }

        Ok(Self {
            name,
            specified_type,
            value,
        })
    }
}

#[derive(Clone)]
pub struct FunctionSignature {
    arguments: Vec<DefinitionParameter>,
    return_type: Type,
}

impl FunctionSignature {
    pub fn arguments(&self) -> &Vec<DefinitionParameter> {
        &self.arguments
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }
}

impl Parse for FunctionSignature {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut arguments = Vec::new();
        let mut return_type = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::definition_parameter => {
                    arguments.push(DefinitionParameter::parse(pair)?);
                }
                Rule::r#type => return_type = Some(Type::parse(pair)?),
                rule => unreachable!("Unexpected token in function signature: {:?}", rule),
            }
        }

        Ok(Self {
            arguments,
            return_type: return_type.unwrap(),
        })
    }
}

#[derive(Clone)]
pub struct Assignment {
    name: Identifier,
    specified_type: Option<Type>,
    value: Expression,
}

impl Assignment {
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn specified_type(&self) -> Option<&Type> {
        self.specified_type.as_ref()
    }

    pub fn value(&self) -> Expression {
        // TODO Return reference here
        self.value.clone()
    }
}

impl Parse for Assignment {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut value = Expression::default();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::r#type => {
                    specified_type = Some(Type::parse(pair)?);
                }
                Rule::expression => {
                    value = Expression::parse(pair)?;
                }
                rule => {
                    unreachable!("Unexpected token in assignment: {:?}", rule);
                }
            }
        }

        Ok(Self {
            name,
            specified_type,
            value,
        })
    }
}

#[derive(Clone)]
pub enum FunctionStatement {
    Assignment(Assignment),
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
        assert_eq!(
            pair.as_rule(),
            Rule::function_statement,
            "Unexpected rule: {:?}",
            pair.as_rule()
        );
        let mut pairs = pair.into_inner();
        let first = pairs.next().unwrap();
        match first.as_rule() {
            Rule::assignment => Ok(Self::Assignment(Assignment::parse(first)?)),
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
            rule => unreachable!("Unexpected token in function statement: {:?}", rule),
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
        let body = Parser::vec(pairs, FunctionStatement::parse)?;
        Ok(Self {
            name,
            signature,
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::Symbol;

    #[test]
    fn assignment() {
        use crate::eval::*;
        use crate::parser::Parser;
        use crate::parser::Rule;
        let assignment =
            Parser::parse_rule_or_panic::<crate::function::Assignment>(Rule::assignment, "a = 1");

        let context = Context::default();

        assert_eq!(assignment.name(), &"a".into());
        assert_eq!(
            assignment.value().eval(Some(&context)).unwrap().to_string(),
            "1"
        );
        assert!(assignment.specified_type().is_none());
    }

    #[test]
    fn function_signature() {
        use crate::function::FunctionSignature;
        use crate::parser::Parser;
        use crate::parser::Rule;

        let input = "(a: scalar, b: scalar) -> scalar";

        let function_signature =
            Parser::parse_rule_or_panic::<FunctionSignature>(Rule::function_signature, input);

        assert_eq!(function_signature.arguments().len(), 2);
        assert_eq!(
            function_signature.return_type(),
            &crate::lang_type::Type::Scalar
        );
    }

    #[test]
    fn function_declaration() {
        use crate::function::FunctionDefinition;
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
    }

    #[test]
    fn function_eval() {}
}

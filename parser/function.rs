use std::rc::Rc;

use crate::call::EvaluatedCallArgumentList;
use crate::eval::{Context, Eval, Symbol};
use crate::expression::Expression;
use crate::identifier::Identifier;
use crate::parser::*;

use crate::lang_type::Type;
use crate::module::UseStatement;
use crate::value::Value;

#[derive(Debug, Clone)]
pub struct DefinitionParameter {
    #[allow(dead_code)]
    name: Identifier,
    #[allow(dead_code)]
    specified_type: Option<Type>,
    #[allow(dead_code)]
    value: Option<Expression>,
}

impl DefinitionParameter {
    pub fn new(name: Identifier, specified_type: Option<Type>, value: Option<Expression>) -> Self {
        Self {
            name,
            specified_type,
            value,
        }
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn specified_type(&self) -> Option<&Type> {
        self.specified_type.as_ref()
    }

    pub fn value(&self) -> Option<&Expression> {
        self.value.as_ref()
    }
}

impl std::fmt::Display for DefinitionParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.specified_type, &self.value) {
            (Some(t), Some(v)) => write!(f, "{}: {} = {}", self.name, t, v)?,
            (Some(t), None) => write!(f, "{}: {}", self.name, t)?,
            (None, Some(v)) => write!(f, "{} = {}", self.name, v)?,
            _ => {}
        }

        write!(f, "{}", self.name)
    }
}

impl Parse for DefinitionParameter {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut value = None;

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
    pub parameters: Vec<DefinitionParameter>,
    pub return_type: Type,
}

impl FunctionSignature {
    pub fn parameters(&self) -> &Vec<DefinitionParameter> {
        &self.parameters
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }

    pub fn get_parameter_by_name(&self, name: &Identifier) -> Option<&DefinitionParameter> {
        self.parameters.iter().find(|arg| arg.name() == name)
    }
}

impl Parse for FunctionSignature {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut parameters = Vec::new();
        let mut return_type = None;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::definition_parameter_list => {
                    parameters = Parser::vec(pair.into_inner(), DefinitionParameter::parse)?;
                }
                Rule::r#type => return_type = Some(Type::parse(pair)?),
                rule => unreachable!("Unexpected token in function signature: {:?}", rule),
            }
        }

        Ok(Self {
            parameters,
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

impl Eval for Assignment {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output, crate::eval::Error> {
        let value = self.value.eval(context)?;
        context.add_symbol(Symbol::Value(self.name.clone(), value));
        Ok(())
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

pub type BuiltinFunction =
    Rc<dyn Fn(EvaluatedCallArgumentList, &mut Context) -> Result<Value, crate::eval::Error>>;

#[derive(Clone)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub signature: FunctionSignature,
    pub body: Vec<FunctionStatement>,
    pub builtin: Option<BuiltinFunction>,
}

impl FunctionDefinition {
    pub fn builtin(
        name: Identifier,
        signature: FunctionSignature,
        builtin: BuiltinFunction,
    ) -> Self {
        Self {
            name,
            signature,
            body: Vec::new(),
            builtin: Some(builtin),
        }
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn signature(&self) -> &FunctionSignature {
        &self.signature
    }

    pub fn body(&self) -> &Vec<FunctionStatement> {
        &self.body
    }
}

impl FunctionDefinition {
    pub fn new(
        name: Identifier,
        signature: FunctionSignature,
        body: Vec<FunctionStatement>,
    ) -> Self {
        Self {
            name,
            signature,
            body,
            builtin: None,
        }
    }

    pub fn call(
        &self,
        args: EvaluatedCallArgumentList,
        context: &mut Context,
    ) -> Result<Value, crate::eval::Error> {
        if let Some(builtin) = &self.builtin {
            return builtin(args, context);
        }

        let params = self.signature.parameters();

        for param in params {
            match args.get_named_arg(&param.name) {
                Some(value) => context.add_symbol(Symbol::Value(param.name.clone(), value.clone())),
                None => {
                    return Err(crate::eval::Error::FunctionCallMissingArgument(
                        param.name.clone(),
                    ))
                }
            }
        }

        for statement in self.body.iter() {
            match statement {
                FunctionStatement::Assignment(assignment) => assignment.eval(context)?,
                FunctionStatement::Return(expr) => return expr.eval(context),
                FunctionStatement::FunctionDefinition(f) => f.eval(context)?,
                _ => unimplemented!(),
            }
        }

        Err(crate::eval::Error::FunctionCallMissingReturn)
    }
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
            builtin: None,
        })
    }
}

impl Eval for FunctionDefinition {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output, crate::eval::Error> {
        context.add_symbol(Symbol::Function(self.clone()));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::{Context, Symbol};

    #[test]
    fn assignment() {
        use crate::eval::*;
        use crate::parser::Parser;
        use crate::parser::Rule;
        let assignment =
            Parser::parse_rule_or_panic::<crate::function::Assignment>(Rule::assignment, "a = 1");

        let mut context = Context::default();

        assert_eq!(assignment.name(), &"a".into());
        assert_eq!(
            assignment.value().eval(&mut context).unwrap().to_string(),
            "1"
        );
        assert!(assignment.specified_type().is_none());

        assignment.eval(&mut context).unwrap();

        assert_eq!(context.get_symbol("a").unwrap().name(), "a");
    }

    #[test]
    fn function_signature() {
        use crate::function::FunctionSignature;
        use crate::parser::Parser;
        use crate::parser::Rule;

        let input = "(a: scalar, b: scalar) -> scalar";

        let function_signature =
            Parser::parse_rule_or_panic::<FunctionSignature>(Rule::function_signature, input);

        assert_eq!(function_signature.parameters().len(), 2);
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
        Parser::parse_rule_or_panic::<FunctionDefinition>(Rule::function_definition, input);
    }

    #[test]
    fn function_eval() {
        use crate::eval::Eval;
        use crate::parser::Parser;
        use crate::parser::Rule;

        let input = "function test(a: scalar, b: scalar) -> scalar {
            c = 1.0;
            return a + b + c;
        }";

        let function_def = Parser::parse_rule_or_panic::<crate::function::FunctionDefinition>(
            Rule::function_definition,
            input,
        );

        let mut context = Context::default();
        context.add_symbol(Symbol::Function(function_def));

        let input = "test(a = 1, b = 2)";
        let expr =
            Parser::parse_rule_or_panic::<crate::expression::Expression>(Rule::expression, input);

        let value = expr.eval(&mut context).unwrap();
        assert_eq!(value.to_string(), "4");
    }
}

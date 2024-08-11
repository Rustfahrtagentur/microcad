use super::{call::*, expression::*, identifier::*, lang_type::*, module::*, value::*};
use crate::{eval::*, parser::*, with_pair_ok};

#[derive(Clone)]
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
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut value = None;

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?.value().clone();
                }
                Rule::r#type => {
                    specified_type = Some(Type::parse(pair)?.value().clone());
                }
                Rule::expression => {
                    value = Some(Expression::parse(pair)?.value().clone());
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

        with_pair_ok!(
            Self {
                name,
                specified_type,
                value,
            },
            pair
        )
    }
}

#[derive(Clone)]
pub struct FunctionSignature {
    pub parameters: Vec<DefinitionParameter>,
    pub return_type: Option<Type>,
}

impl FunctionSignature {
    pub fn parameters(&self) -> &Vec<DefinitionParameter> {
        &self.parameters
    }

    pub fn return_type(&self) -> &Option<Type> {
        &self.return_type
    }

    pub fn get_parameter_by_name(&self, name: &Identifier) -> Option<&DefinitionParameter> {
        self.parameters.iter().find(|arg| arg.name() == name)
    }
}

impl Parse for FunctionSignature {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut parameters = Vec::new();
        let mut return_type = None;

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::definition_parameter_list => {
                    parameters = Parser::vec(pair, DefinitionParameter::parse)?
                        .value()
                        .clone();
                }
                Rule::r#type => return_type = Some(Type::parse(pair)?.value().clone()),
                rule => unreachable!("Unexpected token in function signature: {:?}", rule),
            }
        }

        with_pair_ok!(
            Self {
                parameters,
                return_type,
            },
            pair
        )
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
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut value = Expression::default();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?.value().clone();
                }
                Rule::r#type => {
                    specified_type = Some(Type::parse(pair)?.value().clone());
                }
                Rule::expression => {
                    value = Expression::parse(pair)?.value().clone();
                }
                rule => {
                    unreachable!("Unexpected token in assignment: {:?}", rule);
                }
            }
        }

        with_pair_ok!(
            Self {
                name,
                specified_type,
                value,
            },
            pair
        )
    }
}

impl Eval for Assignment {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        let value = self.value.eval(context)?;
        context.add_symbol(Symbol::Value(self.name.clone(), value));
        Ok(())
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(f, "{}: {} = {}", self.name, t, self.value),
            None => write!(f, "{} = {}", self.name, self.value),
        }
    }
}

#[derive(Clone)]
pub enum FunctionStatement {
    Assignment(Assignment),
    Use(UseStatement),
    FunctionDefinition(std::rc::Rc<FunctionDefinition>),
    Return(Box<Expression>),
    If {
        condition: Expression,
        if_body: FunctionBody,
        else_body: FunctionBody,
    },
}

impl Parse for FunctionStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::function_statement);

        let mut inner = pair.clone().into_inner();
        let first = inner.next().unwrap();
        let s = match first.as_rule() {
            Rule::assignment => Self::Assignment(Assignment::parse(first)?.value().clone()),
            Rule::use_statement => Self::Use(UseStatement::parse(first)?.value().clone()),
            Rule::function_definition => Self::FunctionDefinition(std::rc::Rc::new(
                FunctionDefinition::parse(first)?.value().clone(),
            )),
            Rule::function_return_statement => {
                Self::Return(Box::new(Expression::parse(first)?.value().clone()))
            }
            Rule::function_if_statement => {
                let mut pairs = first.into_inner();
                let condition = Expression::parse(pairs.next().unwrap())?.value().clone();
                let if_body = FunctionBody::parse(pairs.next().unwrap())?.value().clone();

                match pairs.next() {
                    None => Self::If {
                        condition,
                        if_body,
                        else_body: FunctionBody::default(),
                    },
                    Some(pair) => {
                        let else_body = FunctionBody::parse(pair)?.value().clone();
                        Self::If {
                            condition,
                            if_body,
                            else_body,
                        }
                    }
                }
            }
            rule => unreachable!("Unexpected token in function statement: {:?}", rule),
        };

        with_pair_ok!(s, pair)
    }
}

pub type BuiltinFunctionFunctor =
    dyn Fn(EvaluatedCallArgumentList, &mut Context) -> Result<Value, Error>;

#[derive(Clone)]
pub struct BuiltinFunction {
    pub name: Identifier,
    pub f: &'static BuiltinFunctionFunctor,
}

impl BuiltinFunction {
    pub fn new(name: Identifier, f: &'static BuiltinFunctionFunctor) -> Self {
        Self { name, f }
    }

    pub fn call(
        &self,
        args: EvaluatedCallArgumentList,
        context: &mut Context,
    ) -> Result<Value, Error> {
        (self.f)(args, context)
    }
}

#[derive(Clone, Default)]
pub struct FunctionBody(pub Vec<FunctionStatement>);

impl Parse for FunctionBody {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::function_body);

        let mut body = Vec::new();
        let p = pair.clone();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::function_statement => {
                    body.push(FunctionStatement::parse(pair)?.value().clone());
                }
                Rule::expression => {
                    body.push(FunctionStatement::Return(Box::new(
                        Expression::parse(pair)?.value().clone(),
                    )));
                }
                rule => unreachable!("Unexpected token in function body: {:?}", rule),
            }
        }

        with_pair_ok!(Self(body), p)
    }
}

#[derive(Clone)]
pub struct FunctionDefinition {
    pub name: Identifier,
    pub signature: FunctionSignature,
    pub body: FunctionBody,
}

impl FunctionDefinition {
    pub fn new(name: Identifier, signature: FunctionSignature, body: FunctionBody) -> Self {
        Self {
            name,
            signature,
            body,
        }
    }

    pub fn call(
        &self,
        args: EvaluatedCallArgumentList,
        context: &mut Context,
    ) -> Result<Value, Error> {
        // TODO: Check if the arguments are correct
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

        for statement in self.body.0.iter() {
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
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::function_definition);
        let mut inner = pair.clone().into_inner();
        let name = Identifier::parse(inner.next().unwrap())?.value().clone();
        let signature = FunctionSignature::parse(inner.next().unwrap())?
            .value()
            .clone();
        let body = FunctionBody::parse(inner.next().unwrap())?.value().clone();

        with_pair_ok!(
            Self {
                name,
                signature,
                body,
            },
            pair
        )
    }
}

impl Eval for std::rc::Rc<FunctionDefinition> {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        context.add_symbol(Symbol::Function(self.clone()));
        Ok(())
    }
}

#[test]
fn assignment() {
    let assignment = Parser::parse_rule_or_panic::<Assignment>(Rule::assignment, "a = 1");

    let mut context = Context::default();

    assert_eq!(assignment.name(), "a");
    assert_eq!(
        assignment.value().eval(&mut context).unwrap().to_string(),
        "1"
    );
    assert!(assignment.specified_type().is_none());

    assignment.eval(&mut context).unwrap();

    assert_eq!(
        context.get_symbols(&"a".into()).first().unwrap().name(),
        "a"
    );
}

#[test]
fn function_signature() {
    let input = "(a: scalar, b: scalar) -> scalar";

    let function_signature =
        Parser::parse_rule_or_panic::<FunctionSignature>(Rule::function_signature, input);

    assert_eq!(function_signature.parameters().len(), 2);
    assert_eq!(function_signature.return_type(), &Some(Type::Scalar));
}

#[test]
fn function_declaration() {
    let input = "function test(a: scalar, b: scalar) -> scalar {
            c = 1.0;
            return a + b + c;
        }";
    Parser::parse_rule_or_panic::<FunctionDefinition>(Rule::function_definition, input);
}

#[test]
fn function_evaluate() {
    let input = r#"
        function test(a: scalar, b: scalar) -> scalar {
            c = 1.0;
            return a + b + c;
        }"#;

    let function_def = std::rc::Rc::new(Parser::parse_rule_or_panic::<FunctionDefinition>(
        Rule::function_definition,
        input,
    ));

    let mut context = Context::default();
    context.add_symbol(Symbol::Function(function_def));

    let input = "test(a = 1, b = 2)";
    let expr = Parser::parse_rule_or_panic::<Expression>(Rule::expression, input);

    let value = expr.eval(&mut context).unwrap();
    assert_eq!(value.to_string(), "4");
}

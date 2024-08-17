use super::{
    call::*,
    expression::*,
    identifier::*,
    lang_type::*,
    module::*,
    value::{self, *},
};
use crate::{eval::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
pub struct DefinitionParameter {
    name: Identifier,
    specified_type: Option<Type>,
    default_value: Option<Expression>,
}

impl DefinitionParameter {
    pub fn new(
        name: Identifier,
        specified_type: Option<Type>,
        default_value: Option<Expression>,
    ) -> Self {
        Self {
            name,
            specified_type,
            default_value,
        }
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn specified_type(&self) -> Option<&Type> {
        self.specified_type.as_ref()
    }

    pub fn default_value(&self) -> Option<&Expression> {
        self.default_value.as_ref()
    }
}

impl std::fmt::Display for DefinitionParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.specified_type, &self.default_value) {
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
        let mut default_value = None;

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?.value().clone();
                }
                Rule::r#type => {
                    specified_type = Some(Type::parse(pair)?.value().clone());
                }
                Rule::expression => {
                    default_value = Some(Expression::parse(pair)?.value().clone());
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

        if specified_type.is_none() && default_value.is_none() {
            return Err(ParseError::DefinitionParameterMissingTypeOrValue(
                name.clone(),
            ));
        }

        with_pair_ok!(
            Self {
                name,
                specified_type,
                default_value,
            },
            pair
        )
    }
}

impl Eval for DefinitionParameter {
    type Output = (Option<Value>, Type);

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        match (&self.specified_type, &self.default_value) {
            (Some(specified_type), Some(expr)) => {
                let default_value = expr.eval(context)?;
                if specified_type != &default_value.ty() {
                    Err(Error::DefinitionParameterTypeMismatch(
                        self.name.clone(),
                        specified_type.clone(),
                        default_value.ty(),
                    ))
                } else {
                    Ok((Some(default_value), specified_type.clone()))
                }
            }
            (Some(t), None) => Ok((None, t.clone())),
            (None, Some(expr)) => {
                let default_value = expr.eval(context)?;
                Ok((Some(default_value.clone()), default_value.ty()))
            }
            (None, None) => Err(Error::DefinitionParameterMissingTypeOrValue(
                self.name.clone(),
            )),
        }
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
    dyn Fn(&ArgumentMap, &mut Context) -> Result<Option<Value>, Error>;

#[derive(Clone)]
pub struct BuiltinFunction {
    pub name: Identifier,
    pub signature: FunctionSignature,
    pub f: &'static BuiltinFunctionFunctor,
}

impl std::fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BUILTIN({})", &self.name)
    }
}

impl BuiltinFunction {
    pub fn new(
        name: Identifier,
        signature: FunctionSignature,
        f: &'static BuiltinFunctionFunctor,
    ) -> Self {
        Self { name, signature, f }
    }

    pub fn call(
        &self,
        args: &CallArgumentList,
        context: &mut Context,
    ) -> Result<Option<Value>, Error> {
        let arg_map = args.match_definition_no_type_check(&self.signature.parameters, context)?;
        let result = (self.f)(&arg_map, context)?;

        match (&result, &self.signature.return_type) {
            (Some(result), Some(return_type)) => {
                if result.ty() != *return_type {
                    Err(Error::TypeMismatch {
                        expected: return_type.clone(),
                        found: result.ty(),
                    })
                } else {
                    Ok(Some(result.clone()))
                }
            }
            (Some(result), None) => Ok(Some(result.clone())),
            (None, Some(_)) => Err(Error::FunctionCallMissingReturn),
            _ => Ok(None),
        }
    }
}

#[derive(Clone, Debug, Default)]
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

#[derive(Clone, Debug)]
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
        args: &CallArgumentList,
        context: &mut Context,
    ) -> Result<Option<Value>, Error> {
        // TODO: Check if the arguments are correct
        let params = self.signature.parameters();
        let arg_map = args.match_definition(params, context)?;

        context.push();
        for (name, value) in arg_map.iter() {
            context.add_symbol(Symbol::Value(name.clone(), value.clone()));
        }

        for statement in self.body.0.iter() {
            match statement {
                FunctionStatement::Assignment(assignment) => assignment.eval(context)?,
                FunctionStatement::Return(expr) => return Ok(Some(expr.eval(context)?)),
                FunctionStatement::FunctionDefinition(f) => f.eval(context)?,
                _ => unimplemented!(),
            }
        }
        context.pop();
        Ok(None)
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

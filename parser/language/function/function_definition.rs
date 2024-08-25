use crate::{eval::*, language::*, parser::*, with_pair_ok};

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

    pub fn call(&self, args: &CallArgumentList, context: &mut Context) -> Result<Option<Value>> {
        // TODO: Check if the arguments are correct
        let params = &self.signature.parameters;
        let arg_map = args
            .eval(context)?
            .get_matching_arguments(&params.eval(context)?)?;

        context.push();
        for (name, value) in arg_map.iter() {
            context.add_value(name.clone(), value.clone());
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

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        context.add_function(self.clone());
        Ok(())
    }
}

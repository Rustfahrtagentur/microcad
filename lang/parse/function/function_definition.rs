//! Function definition parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Function definition
#[derive(Clone, Debug)]
pub struct FunctionDefinition {
    /// NAme of the function
    pub name: Identifier,
    /// Function signature
    pub signature: FunctionSignature,
    /// Function body
    pub body: FunctionBody,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for FunctionDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl FunctionDefinition {
    pub fn new(name: Identifier, signature: FunctionSignature, body: FunctionBody) -> Self {
        Self {
            name,
            signature,
            body,
            src_ref: SrcRef(None),
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
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::function_definition);
        let mut inner = pair.clone().into_inner();
        let name = Identifier::parse(inner.next().unwrap())?;
        let signature = FunctionSignature::parse(inner.next().unwrap())?;
        let body = FunctionBody::parse(inner.next().unwrap())?;

        Ok(Self {
            name,
            signature,
            body,
            src_ref: pair.into(),
        })
    }
}

impl Eval for std::rc::Rc<FunctionDefinition> {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        context.add_function(self.clone());
        Ok(())
    }
}

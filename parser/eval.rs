use std::collections::HashMap;

use crate::expression::Expression;
use crate::syntax_tree::SyntaxNode;

#[derive(Debug)]
pub enum Error {
    InvalidOperation,
    InvalidFormatString,
    EvaluateToStringError,
}

// Context for evaluation
pub struct Context {
    node: Option<SyntaxNode>,
    symbols: HashMap<String, SyntaxNode>,
    //    type_registry: HashMap<String, SyntaxNode>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            node: None,
            symbols: HashMap::new(),
        }
    }
}

pub trait Eval: Sized {
    fn eval(self, context: Option<&Context>) -> Result<Box<Expression>, Error>;

    fn eval_to_string(self, context: Option<&Context>) -> Result<String, Error> {
        let result = self.eval(context)?;
        match result.as_ref() {
            Expression::NumberLiteral(n) => Ok(n.to_string()),
            Expression::BoolLiteral(b) => Ok(b.to_string()),
            Expression::StringLiteral(s) => Ok(s.clone()),
            //Expression::ListExpression(list) => list.eval_to_string(context)?,
            _ => Err(Error::EvaluateToStringError),
        }
    }
}

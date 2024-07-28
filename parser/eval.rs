use std::collections::HashMap;

use crate::expression::Expression;
use crate::langtype::Type;
use crate::syntax_tree::SyntaxNode;

#[derive(Debug)]
pub enum Error {
    InvalidOperation,
    InvalidFormatString,
    InvalidType,
    TypeMismatch,
    EvaluateToStringError,
}

// Context for evaluation
pub struct Context {
    node: SyntaxNode,
    symbols: HashMap<String, SyntaxNode>,
    //    type_registry: HashMap<String, SyntaxNode>,
}

impl Context {
    pub fn new(node: SyntaxNode) -> Self {
        Self {
            node,
            symbols: HashMap::new(),
        }
    }
}

pub trait Eval: Sized {
    /// Evaluate the type into an expression
    fn eval(self, context: Option<&Context>) -> Result<Box<Expression>, Error>;

    /// Evaluate the type into a string, TODO remove this and implement Display for Expression
    fn eval_to_string(self, context: Option<&Context>) -> Result<String, Error> {
        let result = self.eval(context)?;
        match result.as_ref() {
            Expression::NumberLiteral(n) => Ok(n.to_string()),
            Expression::BoolLiteral(b) => Ok(b.to_string()),
            Expression::StringLiteral(s) => Ok(s.clone()),
            Expression::ListExpression(list) => list.clone().eval_to_string(context),
            _ => Err(Error::EvaluateToStringError),
        }
    }

    /// The expected destination type after evaluation
    fn eval_type(&self, context: Option<&Context>) -> Result<Type, crate::eval::Error>;
}

pub trait EvalTo<T> {
    fn eval_to(self, context: Option<&Context>) -> Result<T, Error>;
}

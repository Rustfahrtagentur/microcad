//! Format expression parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Format expression including format specification
#[allow(dead_code)]
#[derive(Clone, Default, Debug)]
pub struct FormatExpression(FormatSpec, Box<Expression>);

impl Parse for FormatExpression {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut fo = FormatSpec::default();
        let mut expr = Expression::default();
        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::format_spec => fo = FormatSpec::parse(pair)?,
                Rule::expression => expr = Expression::parse(pair)?,
                _ => unreachable!(),
            }
        }
        Ok(Self(fo, Box::new(expr)))
    }
}

impl SrcReferrer for FormatExpression {
    fn src_ref(&self) -> SrcRef {
        SrcRef::merge(self.0.src_ref(), self.1.src_ref())
    }
}

impl Eval for FormatExpression {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Value> {
        Ok(Value::String(format!("{}", self.1.eval(context)?)))
    }
}

impl std::fmt::Display for FormatExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", self.1)
    }
}

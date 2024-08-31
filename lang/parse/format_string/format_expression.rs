use crate::{eval::*, parse::*, parser::*};

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

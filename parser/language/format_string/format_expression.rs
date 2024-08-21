use crate::{eval::*, language::*, parser::*, with_pair_ok};

#[allow(dead_code)]
#[derive(Clone, Default, Debug)]
pub struct FormatExpression(FormatSpec, Box<Expression>);

impl Parse for FormatExpression {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut fo = FormatSpec::default();
        let mut expr = Expression::default();
        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::format_spec => fo = FormatSpec::parse(pair)?.value().clone(),
                Rule::expression => expr = Expression::parse(pair)?.value().clone(),
                _ => unreachable!(),
            }
        }
        with_pair_ok!(Self(fo, Box::new(expr)), pair)
    }
}

impl Eval for FormatExpression {
    type Output = Value;

    fn eval(&self, context: &mut Context) -> Result<Value, Error> {
        Ok(Value::String(format!("{}", self.1.eval(context)?)))
    }
}

impl std::fmt::Display for FormatExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", self.1)
    }
}

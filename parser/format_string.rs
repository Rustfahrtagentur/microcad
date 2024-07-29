use crate::eval::{Context, Error, Eval};
use crate::expression::Expression;
use crate::parser::*;
use crate::value::Value;

#[derive(Default, Clone)]
struct FormatSpec {
    precision: Option<u32>,
    leading_zeros: Option<u32>,
}

impl Parse for FormatSpec {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut opt = FormatSpec::default();

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::format_spec_precision => {
                    opt.precision = Some(pair.as_span().as_str()[1..].parse().unwrap())
                }
                Rule::format_spec_leading_zeros => {
                    opt.leading_zeros = Some(pair.as_span().as_str()[1..].parse().unwrap())
                }
                _ => unreachable!(),
            }
        }

        Ok(opt)
    }
}

#[derive(Default, Clone)]
pub struct FormatExpression(FormatSpec, Box<Expression>);

impl Parse for FormatExpression {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut fo = FormatSpec::default();
        let mut expr = Expression::default();
        for pair in pair.into_inner() {
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
    fn eval(self, context: Option<&Context>) -> Result<Value, Error> {
        let value = self.1.eval(context)?;
        Ok(Value::String(format!("{}", value)))
    }

    fn eval_type(
        &self,
        context: Option<&Context>,
    ) -> Result<crate::langtype::Type, crate::eval::Error> {
        Ok(crate::langtype::Type::String)
    }
}

#[derive(Clone)]
enum FormatStringInner {
    String(String),
    FormatExpression(FormatExpression),
}

/// Definition and implementation for `StringLiteral`
#[derive(Default, Clone)]
pub struct FormatString(Vec<FormatStringInner>);

impl FormatString {
    pub fn push_string(&mut self, s: String) {
        self.0.push(FormatStringInner::String(s));
    }

    pub fn push_format_expr(&mut self, expr: FormatExpression) {
        self.0.push(FormatStringInner::FormatExpression(expr));
    }

    pub fn section_count(&self) -> usize {
        self.0.len()
    }
}

impl Eval for FormatString {
    fn eval(self, context: Option<&Context>) -> Result<Value, Error> {
        let mut result = String::new();
        for elem in self.0 {
            match elem {
                FormatStringInner::String(s) => result += &s,
                FormatStringInner::FormatExpression(expr) => match expr.eval(context) {
                    Ok(Value::String(s)) => result += &s,
                    Err(e) => return Err(e),
                    _ => unreachable!("FormatExpression should always evaluate to a string"),
                },
            }
        }
        Ok(Value::String(result))
    }

    fn eval_type(
        &self,
        context: Option<&Context>,
    ) -> Result<crate::langtype::Type, crate::eval::Error> {
        Ok(crate::langtype::Type::String)
    }
}

impl Parse for FormatString {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let pairs = pair.into_inner();
        let mut fs = Self::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::string_literal_inner => fs.push_string(pair.as_span().as_str().to_string()),
                Rule::format_expression => fs.push_format_expr(FormatExpression::parse(pair)?),
                _ => unreachable!(),
            }
        }

        Ok(fs)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn format_string() {
        use pest::Parser;
        let pair = crate::parser::Parser::parse(Rule::format_string, "\"A{2 + 4}B\"")
            .unwrap()
            .next()
            .unwrap();

        let s = FormatString::parse(pair).unwrap();
        assert_eq!(s.section_count(), 3);

        let value = s.eval(None).unwrap();

        assert_eq!(value, Value::String("A6B".to_string()));
    }
}

use crate::eval::{Context, Error, Eval};
use crate::expression::Expression;
use crate::literal::NumberLiteral;

#[derive(Default)]
struct FormatOption {
    precision: Option<u32>,
    leading_zeros: Option<u32>,
}

impl crate::Parse for FormatOption {
    fn parse(pair: pest::iterators::Pair<crate::Rule>) -> Result<Self, crate::ParseError> {
        let mut opt = FormatOption::default();
        use crate::Rule;

        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::format_option_precision => {
                    opt.precision = Some(pair.as_span().as_str()[1..].parse().unwrap())
                }
                Rule::format_option_leading_zeros => {
                    opt.leading_zeros = Some(pair.as_span().as_str()[1..].parse().unwrap())
                }
                _ => unreachable!(),
            }
        }

        Ok(opt)
    }
}

struct FormatExpression(FormatOption, Box<Expression>);

impl crate::Parse for FormatExpression {
    fn parse(pair: pest::iterators::Pair<crate::Rule>) -> Result<Self, crate::ParseError> {
        let mut fo = FormatOption::default();
        let mut expr = Expression::default();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                crate::Rule::format_option => fo = FormatOption::parse(pair)?,
                crate::Rule::expression => expr = Expression::parse(pair)?,
                _ => unreachable!(),
            }
        }
        Ok(Self(fo, Box::new(expr)))
    }
}

impl Eval for FormatExpression {
    fn eval(self, context: Option<&Context>) -> Result<Box<Expression>, Error> {
        self.1.eval(context)
    }

    fn eval_to_string(self, context: Option<&Context>) -> Result<String, Error> {
        let expr = self.eval(context)?;
        match expr.as_ref() {
            Expression::NumberLiteral(n) => Ok(format!("{}", n.value()).to_string()), // TODO Consider format options
            _ => expr.eval_to_string(context),
        }
    }
}

enum FormatStringInner {
    String(String),
    FormatExpression(FormatExpression),
}

/// Definition and implementation for `StringLiteral`
#[derive(Default)]
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
    fn eval(self, context: Option<&Context>) -> Result<Box<Expression>, Error> {
        let mut result = String::new();
        for elem in self.0 {
            match elem {
                FormatStringInner::String(s) => result += &s,
                FormatStringInner::FormatExpression(expr) => {
                    result += &expr.eval_to_string(context)?
                }
            }
        }
        Ok(Box::new(Expression::StringLiteral(result)))
    }
}

impl crate::Parse for FormatString {
    fn parse(pair: pest::iterators::Pair<crate::Rule>) -> Result<Self, crate::ParseError> {
        let pairs = pair.into_inner();
        let mut fs = Self::default();
        for pair in pairs {
            match pair.as_rule() {
                crate::Rule::string_literal_inner => {
                    fs.push_string(pair.as_span().as_str().to_string())
                }
                crate::Rule::format_expression => {
                    fs.push_format_expr(FormatExpression::parse(pair)?)
                }
                _ => unreachable!(),
            }
        }

        Ok(fs)
    }
}

#[cfg(test)]
mod tests {
    use crate::Parse;

    use super::*;

    #[test]
    fn format_string() {
        use pest::Parser;
        let pair = crate::CsglParser::parse(crate::Rule::format_string, "\"A{2 + 4}B\"")
            .unwrap()
            .next()
            .unwrap();

        let s = FormatString::parse(pair).unwrap();
        assert_eq!(s.section_count(), 3);

        let s = s.eval_to_string(None).unwrap();

        assert_eq!(&s, "A6B");
    }
}

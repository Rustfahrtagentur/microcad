use crate::eval::{Context, Error, Eval};
use crate::expression::Expression;

enum FormatStringInner {
    String(String),
    FormatExpression(Box<Expression>),
}

/// Definition and implementation for `StringLiteral`
#[derive(Default)]
pub struct FormatString(Vec<FormatStringInner>);

impl FormatString {
    pub fn push_string(&mut self, s: String) {
        self.0.push(FormatStringInner::String(s));
    }

    pub fn push_format_expr(&mut self, expr: Box<Expression>) {
        self.0.push(FormatStringInner::FormatExpression(expr));
    }

    pub fn section_count(&self) -> usize {
        self.0.len()
    }

    pub fn parse_format_expression(
        pair: pest::iterators::Pair<crate::Rule>,
    ) -> Result<Box<Expression>, crate::ParseError> {
        use crate::Parse;
        Expression::parse(pair.into_inner().next().unwrap()).map(Box::new)
    }
}

impl Eval for FormatString {
    fn eval(self, context: &Context) -> Result<Box<Expression>, Error> {
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
                crate::Rule::string_literal_inner => fs.push_string(pair.to_string()),
                crate::Rule::format_expression => {
                    fs.push_format_expr(FormatString::parse_format_expression(pair)?)
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
        let pair = crate::CsglParser::parse(crate::Rule::format_string, "\"A{2.0}B\"")
            .unwrap()
            .next()
            .unwrap();

        let s = FormatString::parse(pair).unwrap();
        assert_eq!(s.section_count(), 3);
    }
}

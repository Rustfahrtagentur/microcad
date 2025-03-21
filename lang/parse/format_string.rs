use crate::{parse::*, parser::*, syntax::*};

impl Parse for FormatExpression {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(Self::new(
            pair.find(Rule::format_spec),
            pair.find(Rule::expression).expect("Missing expression"),
        ))
    }
}

impl Parse for FormatSpec {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut opt = FormatSpec::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::format_spec_precision => {
                    opt.precision = Some(pair.as_span().as_str()[1..].parse()?)
                }
                Rule::format_spec_width => opt.width = Some(pair.as_span().as_str()[1..].parse()?),
                _ => unreachable!(),
            }
        }

        opt.src_ref = pair.into();

        Ok(opt)
    }
}

impl Parse for FormatString {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut fs = Self::default();
        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::string_literal_inner => fs.push_string(pair.as_span().as_str().to_string()),
                Rule::format_expression => fs.push_format_expr(FormatExpression::parse(pair)?),
                _ => unreachable!(),
            }
        }

        Ok(fs)
    }
}

impl std::str::FromStr for FormatString {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Parser::parse_rule::<Self>(Rule::format_string, s, 0)
    }
}

use crate::{
    parser::{Pair, Parse, ParseResult, Rule},
    with_pair_ok,
};

#[derive(Clone, Debug, Default)]
pub struct FormatSpec {
    precision: Option<u32>,
    leading_zeros: Option<u32>,
}

impl Parse for FormatSpec {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut opt = FormatSpec::default();

        for pair in pair.clone().into_inner() {
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

        with_pair_ok!(opt, pair)
    }
}

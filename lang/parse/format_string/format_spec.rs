use crate::{parser::*, src_ref::*};

#[derive(Clone, Debug, Default)]
pub struct FormatSpec {
    precision: Option<u32>,
    leading_zeros: Option<u32>,
    src_ref: SrcRef,
}

impl SrcReferrer for FormatSpec {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Parse for FormatSpec {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
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

        opt.src_ref = pair.into();

        Ok(opt)
    }
}

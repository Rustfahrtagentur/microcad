// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Format Specification parser entity

use crate::{parse::*, parser::*, src_ref::*};

/// Format specification
#[derive(Clone, Debug, Default)]
pub struct FormatSpec {
    /// Precision for number formatting
    precision: Option<u32>,
    /// Alignment width (leading zeros)
    width: Option<u32>,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for FormatSpec {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
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

impl std::fmt::Display for FormatSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.width, self.precision) {
            (Some(width), Some(precision)) => write!(f, "0{width}.{precision}"),
            (None, Some(precision)) => write!(f, ".{precision}"),
            (Some(width), None) => write!(f, "0{width}"),
            _ => Ok(()),
        }
    }
}

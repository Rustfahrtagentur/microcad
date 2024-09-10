// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Format Specification parser entity

use crate::{errors::*, parser::*, src_ref::*};

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
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut opt = FormatSpec::default();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::format_spec_precision => {
                    opt.precision = Some(pair.as_span().as_str()[1..].parse().unwrap())
                }
                Rule::format_spec_leading_zeros => {
                    opt.width = Some(pair.as_span().as_str()[1..].parse().unwrap())
                }
                _ => unreachable!(),
            }
        }

        opt.src_ref = pair.into();

        Ok(opt)
    }
}


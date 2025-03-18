// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Number literal parser entity

use crate::{parse::*, parser::*, r#type::*};
use literal::*;

/// Number literal
#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral(pub f64, pub Unit, SrcRef);

impl NumberLiteral {
    /// Create from usize value
    #[cfg(test)]
    pub fn from_usize(value: usize) -> Self {
        Self(value as f64, Unit::None, SrcRef(None))
    }

    /// Create from integer value
    #[cfg(test)]
    pub fn from_int(value: i64) -> Self {
        Self(value as f64, Unit::None, SrcRef(None))
    }

    /// Returns the actual value of the literal
    pub fn normalized_value(&self) -> f64 {
        self.1.normalize(self.0)
    }

    /// return unit
    pub fn unit(&self) -> Unit {
        self.1
    }
}

impl crate::ty::Ty for NumberLiteral {
    fn ty(&self) -> Type {
        self.1.ty()
    }
}

impl SrcReferrer for NumberLiteral {
    fn src_ref(&self) -> literal::SrcRef {
        self.2.clone()
    }
}

impl Parse for NumberLiteral {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::number_literal);

        let mut inner = pair.inner();
        let number_token = inner.next().expect("Expected number token");

        assert!(
            number_token.as_rule() == Rule::number
                || number_token.as_rule() == Rule::integer_literal
        );

        let value = number_token.as_str().parse::<f64>()?;

        let mut unit = Unit::None;

        if let Some(unit_token) = inner.next() {
            unit = Unit::parse(unit_token)?;
        }
        Ok(NumberLiteral(value, unit, pair.clone().into()))
    }
}

impl std::fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

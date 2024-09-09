//! Number literal parser entity

use crate::{eval::*, parse::*, parser::*, r#type::*};
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
    pub fn value(&self) -> f64 {
        self.1.normalize(self.0)
    }

    /// return unit
    pub fn unit(&self) -> Unit {
        self.1
    }
}

impl Ty for NumberLiteral {
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
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::number_literal);

        let mut inner = pair.clone().into_inner();
        let number_token = inner.next().unwrap();

        assert!(
            number_token.as_rule() == Rule::number
                || number_token.as_rule() == Rule::integer_literal
        );

        let value = number_token.as_str().parse::<f64>()?;

        let mut unit = Unit::None;

        if let Some(unit_token) = inner.next() {
            unit = Unit::parse(unit_token)?;
        }
        Ok(NumberLiteral(value, unit, pair.into()))
    }
}

impl Eval for NumberLiteral {
    type Output = Value;

    fn eval(&self, _: &mut Context) -> std::result::Result<Value, EvalError> {
        match self.1.ty() {
            Type::Scalar => Ok(Value::Scalar(Refer::new(self.0, self.2.clone()))),
            Type::Angle => Ok(Value::Angle(Refer::new(self.0, self.2.clone()))),
            Type::Length => Ok(Value::Length(Refer::new(self.0, self.2.clone()))),
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

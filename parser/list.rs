use crate::eval::{Context, Eval};
use crate::langtype::Type;
use crate::parser::{Pair, Parse, ParseError};
use crate::{expression::Expression, literal::NumberLiteral};

#[derive(Default, Clone)]
pub struct ListExpression(Vec<Expression>);

impl Parse for ListExpression {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut list = Self::default();
        for pair in pair.into_inner() {
            list.0.push(Expression::parse(pair)?);
        }

        Ok(list)
    }
}

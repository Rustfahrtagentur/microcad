use crate::{expression::Expression, literal::NumberLiteral};

pub struct ListExpression(Vec<Box<Expression>>);

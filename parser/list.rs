use crate::{expression::Expression, literal::NumberLiteral};

#[derive(Default, Clone)]
pub struct ListExpression(Vec<Box<Expression>>);

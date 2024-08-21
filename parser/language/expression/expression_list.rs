use crate::{language::*, parser::*, with_pair_ok};

#[derive(Clone, Default, Debug)]
pub struct ExpressionList(Vec<Expression>);

impl std::ops::Deref for ExpressionList {
    type Target = Vec<Expression>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ExpressionList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ExpressionList {
    pub fn new(v: Vec<Expression>) -> Self {
        Self(v)
    }
}

impl IntoIterator for ExpressionList {
    type Item = Expression;
    type IntoIter = std::vec::IntoIter<Expression>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Parse for ExpressionList {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut vec = Vec::new();

        for pair in pair.clone().into_inner() {
            vec.push(Expression::parse(pair)?.value().clone());
        }

        with_pair_ok!(Self(vec), pair)
    }
}

impl std::fmt::Display for ExpressionList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|expr| expr.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
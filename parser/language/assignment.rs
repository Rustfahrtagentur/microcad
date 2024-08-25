use crate::{eval::*, language::*, parser::*, r#type::*, with_pair_ok};

#[derive(Clone, Debug)]
pub struct Assignment {
    name: Identifier,
    specified_type: Option<Type>,
    value: Expression,
}

impl Assignment {
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn specified_type(&self) -> Option<&Type> {
        self.specified_type.as_ref()
    }

    pub fn value(&self) -> Expression {
        // TODO Return reference here
        self.value.clone()
    }
}

impl Parse for Assignment {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut value = Expression::default();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?.value().clone();
                }
                Rule::r#type => {
                    specified_type = Some(Type::parse(pair)?.value().clone());
                }
                Rule::expression => {
                    value = Expression::parse(pair)?.value().clone();
                }
                rule => {
                    unreachable!("Unexpected token in assignment: {:?}", rule);
                }
            }
        }

        with_pair_ok!(
            Self {
                name,
                specified_type,
                value,
            },
            pair
        )
    }
}

impl Eval for Assignment {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let value = self.value.eval(context)?;
        context.add_value(self.name.clone(), value);
        Ok(())
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(f, "{}: {} = {}", self.name, t, self.value),
            None => write!(f, "{} = {}", self.name, self.value),
        }
    }
}

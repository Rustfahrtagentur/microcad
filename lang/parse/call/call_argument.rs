use crate::{eval::*, ord_map::OrdMapValue, parse::*, parser::*};

#[derive(Clone, Debug, Default)]
pub struct CallArgument {
    pub name: Option<Identifier>,
    pub value: Expression,
}

impl Sym for CallArgument {
    fn id(&self) -> Option<microcad_core::Id> {
        if let Some(name) = &self.name {
            name.id()
        } else {
            None
        }
    }
}

impl OrdMapValue<Identifier> for CallArgument {
    fn key(&self) -> Option<Identifier> {
        self.name.clone()
    }
}

impl Parse for CallArgument {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        match pair.clone().as_rule() {
            Rule::call_named_argument => {
                let mut inner = pair.clone().into_inner();
                let first = inner.next().unwrap();
                let second = inner.next().unwrap();

                Ok(CallArgument {
                    name: Some(Identifier::parse(first)?),
                    value: Expression::parse(second)?,
                })
            }
            Rule::expression => Ok(CallArgument {
                name: None,
                value: Expression::parse(pair.clone())?,
            }),
            rule => unreachable!("CallArgument::parse expected call argument, found {rule:?}"),
        }
    }
}

impl Eval for CallArgument {
    type Output = CallArgumentValue;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        Ok(CallArgumentValue {
            name: self.id(),
            value: self.value.eval(context)?,
        })
    }
}

impl std::fmt::Display for CallArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.name {
            Some(ref name) => write!(f, "{} = {}", name, self.value),
            None => write!(f, "{}", self.value),
        }
    }
}

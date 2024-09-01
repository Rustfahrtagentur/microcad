//! A single call argument

use crate::{eval::*, ord_map::*, parse::*, parser::*, src_ref::*};

/// Call argument
#[derive(Clone, Debug, Default)]
pub struct CallArgument {
    /// Name of the argument
    pub name: Option<Identifier>,
    /// Value of the argument
    pub value: Expression,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for CallArgument {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
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
                    src_ref: pair.into(),
                })
            }
            Rule::expression => Ok(CallArgument {
                name: None,
                value: Expression::parse(pair.clone())?,
                src_ref: pair.into(),
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

use crate::{
    eval::*,
    parse::*,
    parser::*,
    r#type::*,
    src_ref::{SrcRef, SrcReferrer},
};

#[derive(Clone, Debug)]
pub struct Assignment {
    name: Identifier,
    specified_type: Option<TypeAnnotation>,
    value: Expression,
    src_ref: SrcRef,
}

impl SrcReferrer for Assignment {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Assignment {
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn specified_type(&self) -> Option<&TypeAnnotation> {
        self.specified_type.as_ref()
    }

    pub fn value(&self) -> Expression {
        // TODO Return reference here
        self.value.clone()
    }
}

impl Parse for Assignment {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut name = Identifier::default();
        let mut specified_type = None;
        let mut value = Expression::default();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::identifier => {
                    name = Identifier::parse(pair)?;
                }
                Rule::r#type => {
                    specified_type = Some(TypeAnnotation::parse(pair)?);
                }
                Rule::expression => {
                    value = Expression::parse(pair)?;
                }
                rule => {
                    unreachable!("Unexpected token in assignment: {:?}", rule);
                }
            }
        }

        Ok(Self {
            name,
            specified_type,
            value,
            src_ref: pair.into(),
        })
    }
}

impl Eval for Assignment {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let value = self.value.eval(context)?;
        context.add_value(self.name.id().expect("nameless lvalue"), value);
        Ok(())
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.specified_type {
            Some(t) => write!(f, "{}: {} = {}", self.name, t.ty(), self.value),
            None => write!(f, "{} = {}", self.name, self.value),
        }
    }
}

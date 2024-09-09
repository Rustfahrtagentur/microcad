//! Parameter list parser entity

use crate::{errors::*, eval::*, ord_map::*, parse::*, parser::*, src_ref::*};

/// Parameter list
#[derive(Clone, Debug, Default)]
pub struct ParameterList(Refer<OrdMap<Identifier, Parameter>>);

impl std::ops::Deref for ParameterList {
    type Target = OrdMap<Identifier, Parameter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SrcReferrer for ParameterList {
    fn src_ref(&self) -> parameter::SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::DerefMut for ParameterList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Parameter>> for ParameterList {
    fn from(value: Vec<Parameter>) -> Self {
        Self(Refer::new(
            OrdMap::<Identifier, Parameter>::from(value.clone()),
            match (value.first(), value.last()) {
                (Some(first), Some(last)) => {
                    SrcRef::merge(first.src_ref.clone(), last.src_ref.clone())
                }
                _ => SrcRef(None),
            },
        ))
    }
}

impl Parse for ParameterList {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::parameter_list);
        let mut parameters = ParameterList::default();

        for pair in pair.clone().into_inner() {
            parameters
                .push(Parameter::parse(pair)?)
                .map_err(ParseError::DuplicateIdentifier)?;
        }

        Ok(parameters)
    }
}

impl Eval for ParameterList {
    type Output = ParameterValueList;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let mut values = ParameterValueList::default();
        for parameter in self.iter() {
            values.push(parameter.eval(context)?).unwrap(); // Unwrap is safe here because we know the parameter is unique
        }

        Ok(values)
    }
}

/// Short cut to create a `ParameterList` instance
#[macro_export]
macro_rules! parameter_list {
    [$($param:expr),*] => {
        vec![
            $($param,)*
        ].into()
    };
    ($($name:ident),*) => {
        microcad_lang::parse::parameter_list![$(microcad_lang::parameter!($name)),*]
    };
    ($($name:ident: $ty:ident),*) => {
        microcad_lang::parse::parameter_list![$(microcad_lang::parameter!($name: $ty)),*]
    };
    ($($name:ident: $ty:ident = $value:expr),*) => {
        microcad_lang::parse::parameter_list![$(microcad_lang::parameter!($name: $ty = $value)),*]
    };
}

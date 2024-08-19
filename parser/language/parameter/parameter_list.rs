use super::{Context, Error, Eval, Pair, Parameter, ParameterValueList, Parse, ParseResult};
use crate::{
    language::parameter::{Identifier, IdentifierListError},
    ord_map::OrdMap,
    parser::{Parser, Rule},
    with_pair_ok,
};

#[derive(Clone, Debug, Default)]
pub struct ParameterList(OrdMap<Identifier, Parameter>);

impl std::ops::Deref for ParameterList {
    type Target = OrdMap<Identifier, Parameter>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ParameterList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Parameter>> for ParameterList {
    fn from(value: Vec<Parameter>) -> Self {
        Self(OrdMap::<Identifier, Parameter>::from(value))
    }
}

impl Parse for ParameterList {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::parameter_list);
        let mut parameters = ParameterList::default();

        for pair in pair.clone().into_inner() {
            parameters
                .push(Parameter::parse(pair)?.value().clone())
                .map_err(IdentifierListError::DuplicateIdentifier)?;
        }

        with_pair_ok!(parameters, pair)
    }
}

impl Eval for ParameterList {
    type Output = ParameterValueList;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        let mut values = ParameterValueList::default();
        for parameter in self.iter() {
            values.push(parameter.eval(context)?).unwrap(); // Unwrap is safe here because we know the parameter is unique
        }

        Ok(values)
    }
}

#[macro_export]
macro_rules! parameter_list {
    [$($param:expr),*] => {
        vec![
            $($param,)*
        ].into()
    };
    ($($name:ident),*) => {
        microcad_parser::language::parameter_list![$(microcad_parser::parameter!($name)),*]
    };
    ($($name:ident: $ty:ident),*) => {
        microcad_parser::language::parameter_list![$(microcad_parser::parameter!($name: $ty)),*]
    };
    ($($name:ident: $ty:ident = $value:expr),*) => {
        microcad_parser::language::parameter_list![$(microcad_parser::parameter!($name: $ty = $value)),*]
    };
}

use crate::{eval::*, ord_map::OrdMap, parse::*, parser::*};

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
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::parameter_list);
        let mut parameters = ParameterList::default();

        for pair in pair.clone().into_inner() {
            parameters
                .push(Parameter::parse(pair)?)
                .map_err(IdentifierListError::DuplicateIdentifier)?;
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
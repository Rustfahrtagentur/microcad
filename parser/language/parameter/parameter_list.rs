use super::{
    Context, Error, Eval, Pair, Parameter, ParameterValueList, Parse, ParseError, ParseResult,
};
use crate::{
    parser::{Parser, Rule},
    with_pair_ok,
};

#[derive(Clone, Debug, Default)]
pub struct ParameterList {
    parameters: Vec<Parameter>,
    by_name: std::collections::HashMap<String, usize>,
}

impl ParameterList {
    pub fn new(parameters: Vec<Parameter>) -> Self {
        let mut by_name = std::collections::HashMap::new();
        for (i, parameter) in parameters.iter().enumerate() {
            by_name.insert(parameter.name.to_string(), i);
        }

        Self {
            parameters,
            by_name,
        }
    }

    pub fn push(&mut self, parameter: Parameter) -> Result<(), ParseError> {
        if self.by_name.contains_key(&parameter.name.to_string()) {
            return Err(ParseError::DuplicateParameter(parameter.name.clone()));
        }

        self.by_name
            .insert(parameter.name.to_string(), self.parameters.len());
        self.parameters.push(parameter);
        Ok(())
    }
}

impl Parse for ParameterList {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::parameter_list);
        let mut parameters = ParameterList::default();

        for pair in pair.clone().into_inner() {
            parameters.push(Parameter::parse(pair)?.value().clone())?;
        }

        with_pair_ok!(parameters, pair)
    }
}

impl Eval for ParameterList {
    type Output = ParameterValueList;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        let mut values = ParameterValueList::default();
        for parameter in &self.parameters {
            values.push(parameter.eval(context)?).unwrap(); // Unwrap is safe here because we know the parameter is unique
        }

        Ok(values)
    }
}

impl std::ops::Deref for ParameterList {
    type Target = Vec<Parameter>;

    fn deref(&self) -> &Self::Target {
        &self.parameters
    }
}

#[macro_export]
macro_rules! parameter_list {
    [$($param:expr),*] => {
        microcad_parser::language::parameter::ParameterList::new(vec![
            $($param,)*
        ])
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

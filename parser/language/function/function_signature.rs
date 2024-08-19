use crate::{
    language::{
        identifier::Identifier,
        lang_type::Type,
        parameter::{Parameter, ParameterList},
    },
    parser::{Pair, Parse, ParseResult, Rule},
    with_pair_ok,
};

#[derive(Clone, Debug)]
pub struct FunctionSignature {
    pub parameters: ParameterList,
    pub return_type: Option<Type>,
}

impl FunctionSignature {
    pub fn new(parameters: ParameterList, return_type: Option<Type>) -> Self {
        Self {
            parameters,
            return_type,
        }
    }

    pub fn get_parameter_by_name(&self, name: &Identifier) -> Option<&Parameter> {
        self.parameters.iter().find(|arg| arg.name == *name)
    }
}

impl Parse for FunctionSignature {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut parameters = ParameterList::default();
        let mut return_type = None;

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::parameter_list => {
                    parameters = ParameterList::parse(pair)?.value().clone();
                }
                Rule::r#type => return_type = Some(Type::parse(pair)?.value().clone()),
                rule => unreachable!("Unexpected token in function signature: {:?}", rule),
            }
        }

        with_pair_ok!(
            Self {
                parameters,
                return_type,
            },
            pair
        )
    }
}

#[macro_export]
macro_rules! function_signature {
    ($parameters:expr) => {
        microcad_parser::language::function::FunctionSignature::new($parameters, None)
    };
    (($parameters:expr) -> $return_type:ident) => {
        microcad_parser::language::function::FunctionSignature::new(
            $parameters,
            Some(Type::$return_type),
        )
    };
    () => {};
}

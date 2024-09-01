use crate::{parse::*, parser::*, r#type::*};

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
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut parameters = ParameterList::default();
        let mut return_type = None;

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::parameter_list => {
                    parameters = ParameterList::parse(pair)?;
                }
                Rule::r#type => return_type = Some(Type::parse(pair)?),
                rule => unreachable!("Unexpected token in function signature: {:?}", rule),
            }
        }

        Ok(Self {
            parameters,
            return_type,
        })
    }
}

#[macro_export]
macro_rules! function_signature {
    ($parameters:expr) => {
        microcad_lang::parse::function::FunctionSignature::new($parameters, None)
    };
    (($parameters:expr) -> $return_type:ident) => {
        microcad_lang::parse::function::FunctionSignature::new(
            $parameters,
            Some(Type::$return_type),
        )
    };
    () => {};
}
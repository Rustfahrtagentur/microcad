use crate::{errors::*, parse::*, parser::*, r#type::*, src_ref::*};

#[derive(Clone, Debug)]
pub struct FunctionSignature {
    pub parameters: ParameterList,
    pub return_type: Option<TypeAnnotation>,
    src_ref: SrcRef,
}

impl SrcReferrer for FunctionSignature {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl FunctionSignature {
    // TODO: Is this function #cfg(test) only
    pub fn new(parameters: ParameterList, return_type: Option<Type>) -> Self {
        Self {
            parameters,
            return_type: return_type.map(|r| TypeAnnotation(r, SrcRef(None))),
            src_ref: SrcRef(None),
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
                Rule::r#type => return_type = Some(TypeAnnotation::parse(pair)?),
                rule => unreachable!("Unexpected token in function signature: {:?}", rule),
            }
        }

        Ok(Self {
            parameters,
            return_type,
            src_ref: pair.into(),
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

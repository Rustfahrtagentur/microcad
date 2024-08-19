use super::FunctionSignature;
use crate::{
    eval::{Context, Error, Eval},
    language::{
        call::{ArgumentMap, CallArgumentList},
        identifier::Identifier,
        lang_type::Ty,
        value::Value,
    },
};

pub type BuiltinFunctionFunctor =
    dyn Fn(&ArgumentMap, &mut Context) -> Result<Option<Value>, Error>;

#[derive(Clone)]
pub struct BuiltinFunction {
    pub name: Identifier,
    pub signature: FunctionSignature,
    pub f: &'static BuiltinFunctionFunctor,
}

impl std::fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BUILTIN({})", &self.name)
    }
}

impl BuiltinFunction {
    pub fn new(
        name: Identifier,
        signature: FunctionSignature,
        f: &'static BuiltinFunctionFunctor,
    ) -> Self {
        Self { name, signature, f }
    }

    pub fn call(
        &self,
        args: &CallArgumentList,
        context: &mut Context,
    ) -> Result<Option<Value>, Error> {
        let arg_map = args
            .eval(context)?
            .get_matching_arguments(&self.signature.parameters.eval(context)?)?;
        let result = (self.f)(&arg_map, context)?;

        match (&result, &self.signature.return_type) {
            (Some(result), Some(return_type)) => {
                if result.ty() != *return_type {
                    Err(Error::TypeMismatch {
                        expected: return_type.clone(),
                        found: result.ty(),
                    })
                } else {
                    Ok(Some(result.clone()))
                }
            }
            (Some(result), None) => Ok(Some(result.clone())),
            (None, Some(_)) => Err(Error::FunctionCallMissingReturn),
            _ => Ok(None),
        }
    }
}

use microcad_render::tree::Node;

use crate::{
    eval::{Context, Error, Eval},
    language::{
        call::{ArgumentMap, CallArgumentList},
        identifier::Identifier,
        parameter::ParameterList,
    },
};

pub type BuiltInModuleFn = dyn Fn(&ArgumentMap, &mut Context) -> Result<Node, Error>;

#[derive(Clone)]
pub struct BuiltinModule {
    pub name: Identifier,
    pub parameters: ParameterList,
    pub f: &'static BuiltInModuleFn,
}

impl std::fmt::Debug for BuiltinModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BUILTIN_MOD({})", &self.name)
    }
}

impl BuiltinModule {
    pub fn new(name: Identifier, parameters: ParameterList, f: &'static BuiltInModuleFn) -> Self {
        Self {
            name,
            parameters,
            f,
        }
    }

    pub fn call(&self, args: &CallArgumentList, context: &mut Context) -> Result<Node, Error> {
        let arg_map = args
            .eval(context)?
            .get_matching_arguments(&self.parameters.eval(context)?)?;
        (self.f)(&arg_map, context)
    }
}

#[macro_export]
macro_rules! builtin_module {
    // This macro is used to create a BuiltinModule from a function
    ($name:ident, $f:expr) => {
        BuiltinModule::new(
            stringify!($name).into(),
            &$f,
        )
    };
    // This macro is used to create a BuiltinModule from a function with no arguments
    ($name:ident) => {
        BuiltinModule::new(
            stringify!($name).into(),
            microcad_parser::language::parameter::ParameterList::default(),
            &|_, ctx| Ok(ctx.append_node($name())),
        )
    };
    ($name:ident($($arg:ident: $type:ident),*)) => {
        BuiltinModule::new(
            stringify!($name).into(),
            microcad_parser::parameter_list![$(microcad_parser::parameter!($arg: $type)),*],
            &|args, ctx| {
                let mut l = |$($arg: $type),*| Ok(ctx.append_node($name($($arg),*)));
                let ($($arg),*) = (
                    $(args.get(&stringify!($arg).into()).unwrap().clone().try_into()?),*
                );
                l($($arg),*)
            },
        )
    };
}

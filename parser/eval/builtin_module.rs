use crate::{eval::*, language::*};
use microcad_render::tree;

pub type BuiltInModuleFn = dyn Fn(&ArgumentMap, &mut Context) -> Result<tree::Node>;

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
    pub fn new(name: &'static str, parameters: ParameterList, f: &'static BuiltInModuleFn) -> Self {
        Self {
            name: name.into(),
            parameters,
            f,
        }
    }

    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn call(&self, args: &CallArgumentList, context: &mut Context) -> Result<tree::Node> {
        let arg_map = args
            .eval(context)?
            .get_matching_arguments(&self.parameters.eval(context)?)?;
        (self.f)(&arg_map, context)
    }
}

pub trait DefineBuiltInModule {
    fn name() -> &'static str;
    fn parameters() -> ParameterList;
    fn node(args: &ArgumentMap) -> Result<tree::Node>;

    fn function() -> &'static BuiltInModuleFn {
        &|args, ctx| Ok(ctx.append_node(Self::node(args)?))
    }

    fn builtin_module() -> BuiltinModule {
        BuiltinModule {
            name: Self::name().into(),
            parameters: Self::parameters(),
            f: Self::function(),
        }
    }
}

#[macro_export]
macro_rules! builtin_module {
    // This macro is used to create a BuiltinModule from a function
    ($name:ident($($arg:ident: $type:ident),*) $f:expr) => {
        BuiltinModule::new(
            stringify!($name).into(),
            microcad_parser::parameter_list![$(microcad_parser::parameter!($arg: $type)),*],
            &|args, ctx| {
                let mut l = |$($arg: $type),*| $f;
                let ($($arg),*) = (
                    $(args.get(&stringify!($arg).into()).unwrap().clone().try_into()?),*
                );
                l($($arg),*)
            }

            ,
        )
    };
    // This macro will create a BuiltinModule from a function with arguments
    ($name:ident($($arg:ident: $type:ident),*)) => {
        microcad_parser::eval::BuiltinModule::new(
            stringify!($name).into(),
            microcad_parser::parameter_list![$(microcad_parser::parameter!($arg: $type)),*],
            &|args, ctx| {
                let mut l = |$($arg: $type),*| Ok(ctx.append_node($name($($arg),*)?));
                let ($($arg),*) = (
                    $(args.get(&stringify!($arg).into()).unwrap().clone().try_into()?),*
                );
                l($($arg),*)
            },
        )
    };
}

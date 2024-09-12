// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin module evaluation entity

use crate::{eval::*, parse::*};
use microcad_render::tree;

/// Builtin module initialization functor
pub type BuiltinModuleFn = dyn Fn(&ArgumentMap, &mut Context) -> Result<tree::Node>;

/// Builtin module
#[derive(Clone)]
pub struct BuiltinModule {
    /// Name of the module
    pub name: Identifier,
    /// Module parameters
    pub parameters: ParameterList,
    /// Module's implicit initialization
    pub f: &'static BuiltinModuleFn,
}

impl CallTrait for BuiltinModule {
    /// Call implicit initialization of this module
    fn call(&self, args: &CallArgumentList, context: &mut Context) -> Result<Option<Value>> {
        let arg_map = args
            .eval(context)?
            .get_matching_arguments(&self.parameters.eval(context)?)?;
        Ok(Some(Value::Node((self.f)(&arg_map, context)?)))
    }
}

impl std::fmt::Debug for BuiltinModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BUILTIN_MOD({})", &self.name)
    }
}

/// Builtin module definition
pub trait BuiltinModuleDefinition {
    /// Get name of the builtin module
    fn name() -> &'static str;
    /// Get parameters of the builtin module (implicit init)
    fn parameters() -> ParameterList;
    /// Create node from argument map
    fn node(args: &ArgumentMap) -> Result<tree::Node>;
    /// Implicit initialization functor
    fn function() -> &'static BuiltinModuleFn {
        &|args, ctx| Ok(ctx.append_node(Self::node(args)?))
    }
    /// Generate builtin module
    fn builtin_module() -> BuiltinModule {
        BuiltinModule {
            name: Self::name().into(),
            parameters: Self::parameters(),
            f: Self::function(),
        }
    }
}

/// Short-cut to create a `BuiltinModule`
#[macro_export]
macro_rules! builtin_module {
    // This macro is used to create a BuiltinModule from a function
    ($name:ident($($arg:ident: $type:ident),*) $f:expr) => {
        BuiltinModule {
            name: stringify!($name).into(),
            parameters: microcad_lang::parameter_list![$(microcad_lang::parameter!($arg: $type)),*],
            f: &|args, ctx| {
                let mut l = |$($arg: $type),*| $f;
                let ($($arg),*) = (
                    $(args.get_value(stringify!($arg))),*
                );
                l($($arg),*)
            },
        }
    };
    // This macro will create a BuiltinModule from a function with arguments
    ($name:ident($($arg:ident: $type:ident),*)) => {
        microcad_lang::eval::BuiltinModule {
            name: stringify!($name).into(),
            parameters: microcad_lang::parameter_list![$(microcad_lang::parameter!($arg: $type)),*],
            f:&|args, ctx| {
                let mut l = |$($arg: $type),*| Ok(ctx.append_node($name($($arg),*)?));
                let ($($arg),*) = (
                    $(args.get_value(stringify!($arg))),*
                );
                l($($arg),*)
            },
        }
    };
}

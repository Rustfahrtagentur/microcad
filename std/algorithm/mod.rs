use boolean_op::*;
use microcad_parser::{
    builtin_module,
    eval::Symbols,
    language::module::{BuiltinModule, ModuleDefinition},
};

use crate::ModuleBuilder;

pub mod boolean_op;

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("algorithm")
        .add_builtin_module(builtin_module!(difference()))
        .add_builtin_module(builtin_module!(intersection()))
        .add_builtin_module(builtin_module!(union()))
        .add_builtin_module(builtin_module!(xor()))
        .build()
}

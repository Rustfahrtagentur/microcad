use boolean_op::*;
use microcad_parser::{
    builtin_module,
    language::module::{BuiltinModule, ModuleDefinition},
};

use crate::ModuleBuilder;

pub mod boolean_op;

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("algorithm")
        .builtin_module(builtin_module!(difference()))
        .builtin_module(builtin_module!(intersection()))
        .builtin_module(builtin_module!(union()))
        .builtin_module(builtin_module!(xor()))
        .build()
}

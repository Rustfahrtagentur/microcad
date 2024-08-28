use crate::ModuleBuilder;
use microcad_parser::{
    builtin_module,
    eval::{EvalError, Symbols},
    language::*,
};
use microcad_render::Node;

pub fn difference() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::difference())
}

pub fn union() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::union())
}

pub fn intersection() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::intersection())
}

pub fn xor() -> Result<Node, EvalError> {
    Ok(microcad_core::algorithm::boolean_op::xor())
}

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("algorithm")
        .add_builtin_module(builtin_module!(difference()))
        .add_builtin_module(builtin_module!(intersection()))
        .add_builtin_module(builtin_module!(union()))
        .add_builtin_module(builtin_module!(xor()))
        .build()
}

use boolean_op::difference;
use microcad_parser::language::module::{BuiltinModule, ModuleDefinition};

use crate::ModuleBuilder;

pub mod boolean_op;

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("geo2d")
        .builtin_module(BuiltinModule {
            name: "difference".into(),
            f: &|_, ctx| {
                ctx.append_node(difference());
            },
        })
        .build()
}

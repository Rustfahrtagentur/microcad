mod math;

use ucad_parser::language::module::*;

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    let mut module = ModuleDefinition::namespace("std".into());
    module.add_module(math::builtin_module());
    //module.add_module(geo2d::builtin_module());
    //module.add_module(algorithm::builtin_module());

    std::rc::Rc::new(module)
}

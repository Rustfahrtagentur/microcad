mod math;

use microcad_parser::eval::*;
use microcad_parser::language::call::PositionalNamedList;
use microcad_parser::language::value::Value;
use microcad_parser::language::{function::*, module::*};

pub struct ModuleBuilder {
    module: ModuleDefinition,
}

impl ModuleBuilder {
    pub fn namespace(name: &str) -> ModuleBuilder {
        Self {
            module: ModuleDefinition::namespace(name.into()),
        }
    }

    pub fn builtin_function(&mut self, f: BuiltinFunction) -> &mut Self {
        self.module.add_symbol(Symbol::BuiltinFunction(f));
        self
    }

    pub fn module(&mut self, m: std::rc::Rc<ModuleDefinition>) -> &mut Self {
        self.module.add_module(m);
        self
    }

    pub fn build(&mut self) -> std::rc::Rc<ModuleDefinition> {
        std::rc::Rc::new(self.module.clone())
    }
}

/// @todo: Check if is possible to rewrite this macro with arbitrary number of arguments
#[macro_export]
macro_rules! arg_1 {
    ($f:ident($name:ident) for $($ty:tt),+) => { BuiltinFunction::new(stringify!($f).into(), &|args, _| {
        match args.arg_1(stringify!(name))? {
            $(Value::$ty($name) => Ok(Value::$ty($name.$f())),)*
            Value::List(v) => {
                let mut result = ValueList::new();
                for x in v.iter() {
                    match x {
                        $(Value::$ty(x) => result.push(Value::$ty(x.$f())),)*
                        _ => return Err(Error::InvalidArgumentType(x.ty())),
                    }
                }
                Ok(Value::List(List(result, v.ty())))
            }
            v => Err(Error::InvalidArgumentType(v.ty())),
        }
    })
    };
    ($f:ident($name:ident) $inner:expr) => {
        BuiltinFunction::new(stringify!($f).into(), &|args, _| {
            let l = |$name| $inner;
            l(args.arg_1(stringify!($name))?.clone())
    })
}
}

#[macro_export]
macro_rules! arg_2 {
    ($f:ident($x:ident, $y:ident) $inner:expr) => {
        BuiltinFunction::new(stringify!($f).into(), &|args, _| {
            let l = |$x, $y| $inner;
            let (x, y) = args.arg_2(stringify!($x), stringify!($y))?;
            l(x.clone(), y.clone())
        })
    };
}

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("std")
        .module(math::builtin_module())
        .builtin_function(BuiltinFunction::new("assert".into(), &|args, _| {
            assert!(args[0].into_bool()?);
            unreachable!()
        }))
        .build()
}

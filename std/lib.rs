mod algorithm;
mod geo2d;
mod math;

use microcad_parser::eval::*;
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

    pub fn builtin_module(&mut self, m: BuiltinModule) -> &mut Self {
        self.module.add_symbol(Symbol::BuiltinModule(m));
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
        .module(geo2d::builtin_module())
        .module(algorithm::builtin_module())
        .builtin_function(BuiltinFunction::new("assert".into(), &|args, _| {
            assert!(args[0].into_bool()?);
            Ok(args[0].clone())
        }))
        .build()
}

#[test]
fn test_assert() {
    use microcad_parser::{language::document::Document, parser};
    let doc = match parser::Parser::parse_rule::<Document>(
        parser::Rule::document,
        r#"
            std::assert(std::math::abs(-1.0) == 1.0);
            "#,
    ) {
        Ok(doc) => doc,
        Err(err) => panic!("ERROR: {err}"),
    };

    let mut context = Context::default();
    context.add_symbol(Symbol::ModuleDefinition(builtin_module()));

    if let Err(err) = doc.eval(&mut context) {
        println!("{err}");
    }
}

#[test]
fn difference_svg() {
    use crate::algorithm;
    use microcad_render::svg::SvgRenderer;
    use microcad_render::Renderer;

    let difference = algorithm::boolean_op::difference();
    difference.append(crate::geo2d::circle(4.0));
    difference.append(crate::geo2d::circle(2.0));

    let mut file = std::fs::File::create("difference.svg").unwrap();
    let mut renderer = SvgRenderer::new(&mut file).unwrap();

    renderer.render(difference);
}

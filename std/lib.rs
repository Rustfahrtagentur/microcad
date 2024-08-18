mod algorithm;
mod geo2d;
mod math;

use microcad_parser::eval::*;
use microcad_parser::function_signature;
use microcad_parser::language::lang_type::Type;
use microcad_parser::language::parameter::Parameter;
use microcad_parser::language::value::Value;
use microcad_parser::language::{function::*, module::*};
use microcad_parser::parameter;
use microcad_parser::parameter_list;
use microcad_render::tree::{self, Depth, Node, NodeInner};
use microcad_render::Renderer;

pub struct ModuleBuilder {
    module: ModuleDefinition,
}

impl ModuleBuilder {
    pub fn namespace(name: &str) -> ModuleBuilder {
        Self {
            module: ModuleDefinition::namespace(name.into()),
        }
    }

    pub fn value(&mut self, name: &str, value: Value) -> &mut Self {
        self.module.add_symbol(Symbol::Value(name.into(), value));
        self
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
    ($f:ident($name:ident) for $($ty:tt),+) => { BuiltinFunction::new(
        stringify!($f).into(),
        microcad_parser::function_signature!(microcad_parser::parameter_list!(microcad_parser::parameter!($name))),
        &|args, _| {
        match args.get(&stringify!($name).into()).unwrap() {
            $(Value::$ty($name) => Ok(Some(Value::$ty($name.$f()))),)*
            Value::List(v) => {
                let mut result = ValueList::new();
                for x in v.iter() {
                    match x {
                        $(Value::$ty(x) => result.push(Value::$ty(x.$f())),)*
                        _ => return Err(Error::InvalidArgumentType(x.ty())),
                    }
                }
                Ok(Some(Value::List(List(result, v.ty()))))
            }
            v => Err(Error::InvalidArgumentType(v.ty())),
        }
    })
    };
    ($f:ident($name:ident) $inner:expr) => {
        BuiltinFunction::new(stringify!($f).into(),
        microcad_parser::function_signature!(microcad_parser::parameter_list!(microcad_parser::parameter!($name))),
        &|args, _| {
            let l = |$name| Ok(Some($inner?));
            l(args.get(&stringify!($name).into()).unwrap().clone())
    })
}
}

#[macro_export]
macro_rules! arg_2 {
    ($f:ident($x:ident, $y:ident) $inner:expr) => {
        BuiltinFunction::new(
            stringify!($f).into(),
            microcad_parser::function_signature!(microcad_parser::parameter_list!(
                microcad_parser::parameter!($x),
                microcad_parser::parameter!($y)
            )),
            &|args, _| {
                let l = |$x, $y| Ok(Some($inner?));
                let (x, y) = (
                    args.get(&stringify!($x).into()).unwrap().clone(),
                    args.get(&stringify!($y).into()).unwrap().clone(),
                );
                l(x.clone(), y.clone())
            },
        )
    };
}

pub fn export(filename: String) -> Node {
    Node::new(NodeInner::Export(filename))
}

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("std")
        .module(math::builtin_module())
        .module(geo2d::builtin_module())
        .module(algorithm::builtin_module())
        .builtin_function(BuiltinFunction::new(
            "assert".into(),
            function_signature!(parameter_list!(parameter!(condition: Bool))),
            &|args, _| {
                assert!(args[&"condition".into()].into_bool()?);
                Ok(None)
            },
        ))
        .builtin_module(BuiltinModule {
            name: "export".into(),
            parameters: parameter_list!(parameter!(filename: String)),
            f: &|args, ctx| {
                let filename = args.get(&"filename".into()).unwrap().try_into()?;
                Ok(ctx.append_node(export(filename)))
            },
        })
        .build()
}

#[test]
fn context_namespace() {
    let mut context = Context::default();

    let module = ModuleBuilder::namespace("math")
        .value("pi", Value::Scalar(std::f64::consts::PI))
        .build();

    context.add_symbol(Symbol::ModuleDefinition(module));

    let symbols = context
        .get_symbols_by_qualified_name(&"math::pi".into())
        .unwrap();
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].name(), "pi");
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
    let group = tree::group();
    group.append(crate::geo2d::circle(4.0));
    group.append(crate::geo2d::circle(2.0));
    difference.append(group);

    let mut file = std::fs::File::create("difference.svg").unwrap();
    let mut renderer = SvgRenderer::new(&mut file).unwrap();

    renderer.render(difference);
}

#[test]
fn test_export() {
    use microcad_parser::{language::document::Document, parser};
    let doc = match parser::Parser::parse_rule::<Document>(
        parser::Rule::document,
        r#"
use * from std;

export("export.svg") algorithm::difference() {
    geo2d::circle(radius = 3.0mm);
    geo2d::rect(width = 3.0mm, height = 2.0mm);
};
            "#,
    ) {
        Ok(doc) => doc,
        Err(err) => panic!("ERROR: {err}"),
    };

    let mut context = Context::default();
    context.add_symbol(Symbol::ModuleDefinition(builtin_module()));

    let node = doc.eval(&mut context).unwrap();

    for n in node.descendants() {
        // Indent with depth
        for _ in 0..n.depth() {
            print!("  ");
        }
        println!("{:?}", n);
    }

    for n in node.descendants() {
        let inner = n.borrow();
        if let NodeInner::Export(ref filename) = *inner {
            let mut file = std::fs::File::create(filename).unwrap();
            let mut renderer = microcad_render::svg::SvgRenderer::new(&mut file).unwrap();
            renderer.render(n.clone());
        }
    }
}

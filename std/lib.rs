mod algorithm;
mod context_builder;
mod export;
mod geo2d;

#[cfg(feature = "geo3d")]
mod geo3d;

mod math;

use microcad_lang::parameter;
use microcad_lang::parameter_list;
use microcad_lang::{builtin_module, eval::*, function_signature, parse::*};

pub use context_builder::ContextBuilder;
pub use export::export;

/// Module builder
pub struct ModuleBuilder {
    /// Module definition
    module: ModuleDefinition,
}

impl ModuleBuilder {
    /// Create new module
    pub fn new(name: &str) -> ModuleBuilder {
        Self {
            module: ModuleDefinition::new(name.into()),
        }
    }

    /// Add a value
    pub fn add_value(&mut self, name: &str, value: Value) -> &mut Self {
        self.module.add_value(name.into(), value);
        self
    }

    /// Build module builder
    pub fn build(&mut self) -> std::rc::Rc<ModuleDefinition> {
        std::rc::Rc::new(self.module.clone())
    }
}

impl Symbols for ModuleBuilder {
    fn find_symbols(&self, name: &microcad_core::Id) -> Vec<&Symbol> {
        self.module.find_symbols(name)
    }

    fn add_symbol(&mut self, symbol: Symbol) -> &mut Self {
        self.module.add_symbol(symbol);
        self
    }

    fn copy_symbols<T: Symbols>(&self, into: &mut T) {
        self.module.copy_symbols(into)
    }
}

/// @todo: Check if is possible to rewrite this macro with arbitrary number of arguments
#[macro_export]
macro_rules! arg_1 {
    ($f:ident($name:ident) for $($ty:tt),+) => { BuiltinFunction::new(
        stringify!($f).into(),
        microcad_lang::function_signature!(microcad_lang::parameter_list![microcad_lang::parameter!($name)]),
        &|args, _| {
        match args.get(stringify!($name)).unwrap() {
            $(Value::$ty($name) => Ok(Some(Value::$ty($name.$f()))),)*
            Value::List(v) => {
                let mut result = ValueList::new();
                for x in v.iter() {
                    match x {
                        $(Value::$ty(x) => result.push(Value::$ty(x.$f())),)*
                        _ => return Err(EvalError::InvalidArgumentType(x.ty())),
                    }
                }
                Ok(Some(Value::List(List(result, v.ty()))))
            }
            v => Err(EvalError::InvalidArgumentType(v.ty())),
        }
    })
    };
    ($f:ident($name:ident) $inner:expr) => {
        BuiltinFunction::new(stringify!($f).into(),
        microcad_lang::function_signature!(microcad_lang::parameter_list![microcad_lang::parameter!($name)]),
        &|args, _| {
            let l = |$name| Ok(Some($inner?));
            l(args.get(stringify!($name)).unwrap().clone())
    })
}
}

#[macro_export]
macro_rules! arg_2 {
    ($f:ident($x:ident, $y:ident) $inner:expr) => {
        BuiltinFunction::new(
            stringify!($f).into(),
            microcad_lang::function_signature!(microcad_lang::parameter_list![
                microcad_lang::parameter!($x),
                microcad_lang::parameter!($y)
            ]),
            &|args, _| {
                let l = |$x, $y| Ok(Some($inner?));
                let (x, y) = (
                    args.get(stringify!($x)).unwrap().clone(),
                    args.get(stringify!($y)).unwrap().clone(),
                );
                l(x.clone(), y.clone())
            },
        )
    };
}

use microcad_core::ExportSettings;

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::new("std")
        // TODO: is this correct= Shouldn't this use add_builtin_module() =
        .add_module(math::builtin_module())
        .add_module(geo2d::builtin_module())
        .add_module(geo3d::builtin_module())
        .add_module(algorithm::builtin_module())
        .add_builtin_function(BuiltinFunction::new(
            "assert".into(),
            function_signature!(parameter_list![
                parameter!(condition: Bool),
                parameter!(message: String = "Assertion failed")
            ]),
            &|args, ctx| {
                let message: String = args["message"].clone().try_into()?;
                let condition: bool = args["condition"].clone().try_into()?;
                if !condition {
                    use microcad_lang::diagnostics::AddDiagnostic;
                    ctx.error(
                        microcad_lang::src_ref::SrcRef(None), // TODO: This should be the source reference of the assert function call
                        format!("Assertion failed: {message}"),
                    );
                    Err(EvalError::AssertionFailed(message))
                } else {
                    Ok(None)
                }
            },
        ))
        .add_builtin_module(builtin_module!(export(filename: String) {
            let export_settings = ExportSettings::with_filename(filename.clone());

            Ok(microcad_core::export::export(export_settings))
        }))
        .build()
}

#[test]
fn context_namespace() {
    let mut context = Context::default();

    let module = ModuleBuilder::new("math")
        .add_value("pi", Value::Scalar(std::f64::consts::PI))
        .build();

    context.add_module(module);

    let symbols = context
        .get_symbols_by_qualified_name(&"math::pi".into())
        .unwrap();
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].id().unwrap(), "pi");
}

#[test]
fn test_assert() {
    use microcad_lang::parse::source_file::SourceFile;

    use std::str::FromStr;
    let source_file = match SourceFile::from_str(
        r#"
            std::assert(std::math::abs(-1.0) == 1.0);
        "#,
    ) {
        Ok(source_file) => source_file,
        Err(err) => panic!("ERROR: {err}"),
    };

    let mut context = ContextBuilder::new(source_file).with_std().build();

    match context.eval() {
        Ok(_) => {
            println!("Our assertion was successful as expected");
        }
        Err(err) => panic!("{err}"),
    }
}

#[test]
fn difference_svg() {
    use crate::algorithm;
    use microcad_lang::args;
    use microcad_lang::eval::ArgumentMap;
    use microcad_render::svg::SvgRenderer;
    use microcad_render::Renderer2D;

    let difference = algorithm::difference().unwrap();
    let group = microcad_render::tree::group();
    group.append(crate::geo2d::Circle::node(args!(radius: Scalar = 4.0)).unwrap());
    group.append(crate::geo2d::Circle::node(args!(radius: Scalar = 2.0)).unwrap());
    difference.append(group);

    let file = std::fs::File::create("../test_output/std/difference.svg").unwrap();
    let mut renderer = SvgRenderer::default();
    renderer.set_output(Box::new(file)).unwrap();
    renderer.render_node(difference).unwrap();
}

#[test]
fn difference_stl() {
    use crate::algorithm;
    use microcad_export::stl::StlExporter;
    use microcad_lang::args;
    use microcad_lang::eval::ArgumentMap;

    let difference = algorithm::difference().unwrap();
    let group = microcad_render::tree::group();
    group.append(
        crate::geo3d::Cube::node(
            args!(size_x: Scalar = 4.0, size_y: Scalar = 4.0, size_z: Scalar = 4.0),
        )
        .unwrap(),
    );
    group.append(crate::geo3d::Sphere::node(args!(radius: Scalar = 2.0)).unwrap());
    difference.append(group);

    use microcad_export::Exporter;
    let mut exporter = StlExporter::from_settings(&microcad_core::ExportSettings::with_filename(
        "../test_output/std/difference.stl".to_string(),
    ))
    .unwrap();

    exporter.export(difference).unwrap();
}

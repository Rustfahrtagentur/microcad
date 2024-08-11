use microcad_parser::eval::*;
use microcad_parser::language::lang_type::Ty;
use microcad_parser::language::{function::*, module::*, value::*};

struct ModuleBuilder {
    module: ModuleDefinition,
}

impl ModuleBuilder {
    pub fn namespace(name: &str) -> ModuleBuilder {
        Self {
            module: ModuleDefinition::namespace(name.into()),
        }
    }

    pub fn builtin_function(
        &mut self,
        name: &str,
        f: &'static BuiltinFunctionFunctor,
    ) -> &mut Self {
        self.module
            .add_symbol(Symbol::BuiltinFunction(BuiltinFunction {
                name: name.into(),
                f,
            }));
        self
    }

    pub fn build(&mut self) -> std::rc::Rc<ModuleDefinition> {
        std::rc::Rc::new(self.module.clone())
    }
}

macro_rules! arg_1 {
    ($args:expr, $name:ident, $f:ident, $($ty:tt),+) => {
        match $args.arg_1(stringify!(name))? {
            $(Value::$ty($name) => Ok(Value::$ty($name.$f())),)*
            v => Err(Error::InvalidArgumentType(v.ty())),
        }
    };
}

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("math")
        // abs(x): Absolute value of x
        .builtin_function("abs", &|args, _| {
            arg_1!(args, x, abs, Scalar, Length, Angle, Integer)
        })
        // sin(x): Sine of x
        .builtin_function("sin", &|args, _| arg_1!(args, x, sin, Scalar, Angle))
        .build()
}

#[test]
fn test_build_math_module() {
    use microcad_parser::language::expression::*;
    use microcad_parser::parser::*;

    let module = builtin_module();
    assert_eq!(&module.name, "math");

    let mut context = Context::default();

    context.add_symbol(Symbol::ModuleDefinition(module));

    let input = "math::abs(-1.0)";
    let expr = Parser::parse_rule_or_panic::<Expression>(Rule::expression, input);

    let value = expr.eval(&mut context).unwrap();
    assert_eq!(value.to_string(), "1");
}

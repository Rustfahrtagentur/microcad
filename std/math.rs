use cgmath::InnerSpace;
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
    ($f:ident($name:ident) for $($ty:tt),+) => { &|args, _| {
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
    }
    };
    ($f:ident($name:ident) $inner:expr) => {
        &|args, _| {
            let l = |$name| $inner;
            l(args.arg_1(stringify!($name))?.clone())
    }
}
}

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("math")
        // abs(x): Absolute value of x
        .builtin_function("abs", arg_1!(abs(x) for Scalar, Length, Angle, Integer))
        // sign(x): Sign of x
        .builtin_function(
            "sign",
            arg_1!(sign(x) {
                match x {
                    Value::Scalar(x) => Ok(Value::Scalar(x.signum())),
                    Value::Length(x) => Ok(Value::Scalar(x.signum())),
                    Value::Angle(x) => Ok(Value::Scalar(x.signum())),
                    Value::Integer(x) => Ok(Value::Integer(x.signum())),
                    _ => Err(Error::InvalidArgumentType(x.ty())),
                }
            }),
        )
        // floor(x): Floor of x
        .builtin_function("floor", arg_1!(floor(x) for Scalar, Length, Angle))
        // ceil(x): Ceiling of x
        .builtin_function("ceil", arg_1!(ceil(x) for Scalar, Length, Angle))
        // round(x): Round of x
        .builtin_function("round", arg_1!(round(x) for Scalar, Length, Angle))
        // to_int(x): Convert x to integer
        .builtin_function(
            "to_int",
            arg_1!(to_int(x) {
                match x {
                    Value::Scalar(x) => Ok(Value::Integer(x as i64)),
                    Value::Length(x) => Ok(Value::Integer(x as i64)),
                    Value::Angle(x) => Ok(Value::Integer(x as i64)),
                    Value::Integer(x) => Ok(Value::Integer(x)),
                    _ => Err(Error::InvalidArgumentType(x.ty())),
                }
            }),
        )
        // sin(x): Sine of x
        .builtin_function("sin", arg_1!(sin(x) for Scalar, Angle))
        // cos(x): Cosine of x
        .builtin_function("cos", arg_1!(cos(x) for Scalar, Angle))
        // tan(x): Tangent of x
        .builtin_function("tan", arg_1!(tan(x) for Scalar, Angle))
        // asin(x): Arcsine of x
        .builtin_function(
            "asin",
            arg_1!(asin(x) {
                match x {
                    Value::Scalar(x) => Ok(Value::Angle(x.asin())),
                    _ => Err(Error::InvalidArgumentType(x.ty())),
                }
            }),
        )
        // acos(x): Arccosine of x
        .builtin_function(
            "acos",
            arg_1!(acos(x) {
                match x {
                    Value::Scalar(x) => Ok(Value::Angle(x.acos())),
                    _ => Err(Error::InvalidArgumentType(x.ty())),
                }
            }),
        )
        // atan(x): Arctangent of x
        .builtin_function(
            "atan",
            arg_1!(atan(x) {
                match x {
                    Value::Scalar(x) => Ok(Value::Angle(x.atan())),
                    _ => Err(Error::InvalidArgumentType(x.ty())),
                }
            }),
        )
        // sqrt(x): Square root of x
        .builtin_function("sqrt", arg_1!(sqrt(x) for Scalar))
        // ln(x): Natural logarithm of x
        .builtin_function("ln", arg_1!(ln(x) for Scalar))
        // log2(x): Base 2 logarithm of x
        .builtin_function("log2", arg_1!(log2(x) for Scalar))
        // log10(x): Base 10 logarithm of x
        .builtin_function("log10", arg_1!(log10(x) for Scalar))
        // exp(x): Exponential of x
        .builtin_function("exp", arg_1!(exp(x) for Scalar))
        // length(x): Length of x
        .builtin_function(
            "length",
            arg_1!(length(v) {
                match v {
                    Value::Vec2(v) => Ok(Value::Length(v.magnitude())),
                    Value::Vec3(v) => Ok(Value::Length(v.magnitude())),
                    Value::Vec4(v) => Ok(Value::Length(v.magnitude())),
                    _ => Err(Error::InvalidArgumentType(v.ty())),
                }
            }),
        )
        // normalize(x): Normalize x
        .builtin_function("normalize", arg_1!(normalize(v) for Vec2, Vec3, Vec4))
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

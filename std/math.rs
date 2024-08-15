use crate::{arg_1, arg_2, ModuleBuilder};
use cgmath::InnerSpace;
use microcad_parser::eval::*;
use microcad_parser::language::lang_type::Ty;
use microcad_parser::language::{function::*, module::*, value::*};

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::namespace("math")
        // abs(x): Absolute value of x
        .builtin_function(arg_1!(abs(x) for Scalar, Length, Angle, Integer))
        // sign(x): Sign of x
        .builtin_function(arg_1!(sign(x) {
            match x {
                Value::Scalar(x) | Value::Length(x) | Value::Angle(x) => Ok(Value::Scalar(x.signum())),
                Value::Integer(x) => Ok(Value::Integer(x.signum())),
                _ => Err(Error::InvalidArgumentType(x.ty())),
            }
        }))
        // floor(x): Floor of x
        .builtin_function(arg_1!(floor(x) for Scalar, Length, Angle))
        // ceil(x): Ceiling of x
        .builtin_function(arg_1!(ceil(x) for Scalar, Length, Angle))
        // round(x): Round of x
        .builtin_function(arg_1!(round(x) for Scalar, Length, Angle))
        // to_int(x): Convert x to integer
        .builtin_function(arg_1!(to_int(x) {
            match x {
                Value::Scalar(x) | Value::Length(x) | Value::Angle(x) => Ok(Value::Integer(x as i64)),
                Value::Integer(x) => Ok(Value::Integer(x)),
                _ => Err(Error::InvalidArgumentType(x.ty())),
            }
        }))
        // to_scalar(x): Convert x to scalar
        .builtin_function(arg_1!(to_scalar(x) {
            match x {
                Value::Scalar(x) => Ok(Value::Scalar(x)),
                Value::Length(x) => Ok(Value::Scalar(x)),
                Value::Angle(x) => Ok(Value::Scalar(x)),
                Value::Integer(x) => Ok(Value::Scalar(x as Scalar)),
                _ => Err(Error::InvalidArgumentType(x.ty())),
            }
        }))
        // min(x,y): Minimum of x and y
        .builtin_function(arg_2!(min(x, y) {
            match (x, y) {
                (Value::Scalar(x), Value::Scalar(y)) => Ok(Value::Scalar(x.min(y))),
                (Value::Length(x), Value::Length(y)) => Ok(Value::Length(x.min(y))),
                (Value::Angle(x), Value::Angle(y)) => Ok(Value::Angle(x.min(y))),
                (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x.min(y))),
                (x,_) => Err(Error::InvalidArgumentType(x.ty())),
            }
        }))
        // max(x,y): Maximum of x and y
        .builtin_function(arg_2!(max(x, y) {
            match (x, y) {
                (Value::Scalar(x), Value::Scalar(y)) => Ok(Value::Scalar(x.max(y))),
                (Value::Length(x), Value::Length(y)) => Ok(Value::Length(x.max(y))),
                (Value::Angle(x), Value::Angle(y)) => Ok(Value::Angle(x.max(y))),
                (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x.max(y))),
                (x,_) => Err(Error::InvalidArgumentType(x.ty())),
            }
        }))
        // sin(x): Sine of x
        .builtin_function(arg_1!(sin(x) for Scalar, Angle))
        // cos(x): Cosine of x
        .builtin_function(arg_1!(cos(x) for Scalar, Angle))
        // tan(x): Tangent of x
        .builtin_function(arg_1!(tan(x) for Scalar, Angle))
        // asin(x): Arcsine of x
        .builtin_function(arg_1!(asin(x) {
            match x {
                Value::Scalar(x) => Ok(Value::Angle(x.asin())),
                _ => Err(Error::InvalidArgumentType(x.ty())),
            }
        }))
        // acos(x): Arccosine of x
        .builtin_function(arg_1!(acos(x) {
            match x {
                Value::Scalar(x) => Ok(Value::Angle(x.acos())),
                _ => Err(Error::InvalidArgumentType(x.ty())),
            }
        }))
        // atan(x): Arctangent of x
        .builtin_function(arg_1!(atan(x) {
            match x {
                Value::Scalar(x) => Ok(Value::Angle(x.atan())),
                _ => Err(Error::InvalidArgumentType(x.ty())),
            }
        }))
        // sqrt(x): Square root of x
        .builtin_function(arg_1!(sqrt(x) for Scalar))
        // ln(x): Natural logarithm of x
        .builtin_function(arg_1!(ln(x) for Scalar))
        // log2(x): Base 2 logarithm of x
        .builtin_function(arg_1!(log2(x) for Scalar))
        // log10(x): Base 10 logarithm of x
        .builtin_function(arg_1!(log10(x) for Scalar))
        // exp(x): Exponential of x
        .builtin_function(arg_1!(exp(x) for Scalar))
        // pow(x,y): x raised to the power of y
        .builtin_function(arg_2!(pow(x, y) {
            match (x, y) {
                (Value::Scalar(x), Value::Scalar(y)) => Ok(Value::Scalar(x.powf(y))),
                (Value::Length(x), Value::Scalar(y)) => Ok(Value::Length(x.powf(y))),
                (Value::Angle(x), Value::Scalar(y)) => Ok(Value::Angle(x.powf(y))),
                (Value::Integer(x), Value::Integer(y)) => Ok(Value::Integer(x.pow(y as u32))),
                (Value::Scalar(x), Value::Integer(y)) => Ok(Value::Scalar(x.powf(y as Scalar))),
                (Value::Length(x), Value::Integer(y)) => Ok(Value::Length(x.powf(y as Scalar))),
                (Value::Angle(x), Value::Integer(y)) => Ok(Value::Angle(x.powf(y as Scalar))),
                (x,_) => Err(Error::InvalidArgumentType(x.ty())),
            }
        }))
        // length(x): Length of x
        .builtin_function(arg_1!(length(x) {
            match x {
                Value::Vec2(x) => Ok(Value::Length(x.magnitude())),
                Value::Vec3(x) => Ok(Value::Length(x.magnitude())),
                Value::Vec4(x) => Ok(Value::Length(x.magnitude())),
                _ => Err(Error::InvalidArgumentType(x.ty())),
            }
        }))
        // normalize(x): Normalize x
        .builtin_function(arg_1!(normalize(x) for Vec2, Vec3, Vec4))
        .build()
}

fn test_builtin_function(name: &str, input: &str, expected: &str) {
    use microcad_parser::language::expression::*;
    use microcad_parser::parser::*;

    let module = builtin_module();
    assert_eq!(&module.name, "math");

    let mut context = Context::default();

    context.add_symbol(Symbol::ModuleDefinition(module));

    let expr = Parser::parse_rule_or_panic::<Expression>(Rule::expression, input);

    let value = expr.eval(&mut context).unwrap();
    assert_eq!(value.to_string(), expected, "Failed for '{}'", name);
}

#[test]
fn test_build_math_module() {
    test_builtin_function("abs", "math::abs(-1.0)", "1");
    test_builtin_function("sqrt", "math::sqrt(4.0)", "2");
}

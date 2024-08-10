use super::{function::*, lang_type::*, module::*, value::*};
use crate::eval::*;

#[allow(dead_code)]
fn build_math_module() -> std::rc::Rc<ModuleDefinition> {
    let mut module = ModuleDefinition::namespace("math".into());

    let fn_abs_signature = FunctionSignature {
        parameters: vec![DefinitionParameter::new(
            "x".into(),
            Some(Type::Scalar),
            None,
        )],
        return_type: Type::Scalar,
    };

    let fn_abs = FunctionDefinition::builtin(
        "abs".into(),
        fn_abs_signature,
        std::rc::Rc::new(|args, _| -> Result<Value, Error> {
            let x = args.get_positional_arg(0).unwrap().into_scalar()?;
            Ok(Value::Scalar(x.abs()))
        }),
    );

    module.add_function(fn_abs);

    std::rc::Rc::new(module)
}

#[allow(dead_code)]
fn geo2d_builtin_module() -> std::rc::Rc<ModuleDefinition> {
    let mut module = ModuleDefinition::namespace("geo2d".into());

    let fn_add_signature = FunctionSignature {
        parameters: vec![
            DefinitionParameter::new("x".into(), Some(Type::Scalar), None),
            DefinitionParameter::new("y".into(), Some(Type::Scalar), None),
        ],
        return_type: Type::Scalar,
    };

    let fn_add = FunctionDefinition::builtin(
        "add".into(),
        fn_add_signature,
        std::rc::Rc::new(|args, _| -> Result<Value, Error> {
            let x = args.get_positional_arg(0).unwrap().into_scalar()?;
            let y = args.get_positional_arg(1).unwrap().into_scalar()?;
            Ok(Value::Scalar(x + y))
        }),
    );

    module.add_function(fn_add);

    std::rc::Rc::new(module)
}

#[test]
fn test_build_math_module() {
    use super::expression::*;
    use crate::parser::*;

    let module = build_math_module();
    assert_eq!(module.name, "math".into());

    let mut context = Context::default();

    context.add_symbol(Symbol::ModuleDefinition(module));

    let input = "math::abs(-1.0)";
    let expr = Parser::parse_rule_or_panic::<Expression>(Rule::expression, input);

    let value = expr.eval(&mut context).unwrap();
    assert_eq!(value.to_string(), "1");
}

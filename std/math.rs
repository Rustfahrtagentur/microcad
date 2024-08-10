use ucad_parser::eval::*;
use ucad_parser::language::{function::*, lang_type::*, module::*, value::*};

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
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
            let x = args[0].into_scalar()?;
            Ok(Value::Scalar(x.abs()))
        }),
    );

    module.add_function(fn_abs);

    std::rc::Rc::new(module)
}

#[test]
fn test_build_math_module() {
    use ucad_parser::language::expression::*;
    use ucad_parser::parser::*;

    let module = builtin_module();
    assert_eq!(module.name, "math".into());

    let mut context = Context::default();

    context.add_symbol(Symbol::ModuleDefinition(module));

    let input = "math::abs(-1.0)";
    let expr = Parser::parse_rule_or_panic::<Expression>(Rule::expression, input);

    let value = expr.eval(&mut context).unwrap();
    assert_eq!(value.to_string(), "1");
}

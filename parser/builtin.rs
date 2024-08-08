use std::rc::Rc;

use crate::{
    function::{DefinitionParameter, FunctionDefinition, FunctionSignature},
    lang_type::Type,
    module::ModuleDefinition,
    value::Value,
};

#[allow(dead_code)]
fn build_math_module() -> Rc<ModuleDefinition> {
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
        Rc::new(|args, _| -> Result<Value, crate::eval::Error> {
            let x = args.get_positional_arg(0).unwrap().into_scalar()?;
            Ok(crate::value::Value::Scalar(x.abs()))
        }),
    );

    module.add_function(fn_abs);

    Rc::new(module)
}

#[allow(dead_code)]
fn geo2d_builtin_module() -> Rc<ModuleDefinition> {
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
        Rc::new(|args, _| -> Result<Value, crate::eval::Error> {
            let x = args.get_positional_arg(0).unwrap().into_scalar()?;
            let y = args.get_positional_arg(1).unwrap().into_scalar()?;
            Ok(crate::value::Value::Scalar(x + y))
        }),
    );

    module.add_function(fn_add);

    Rc::new(module)
}

#[cfg(test)]
mod tests {
    use crate::{
        eval::{Eval, Symbol},
        parser::{Parser, Rule},
    };

    use super::*;

    #[test]
    fn test_build_math_module() {
        let module = build_math_module();
        assert_eq!(module.name, "math".into());

        let mut context = crate::eval::Context::default();

        context.add_symbol(Symbol::ModuleDefinition(module));

        let input = "math::abs(-1.0)";
        let expr =
            Parser::parse_rule_or_panic::<crate::expression::Expression>(Rule::expression, input);

        let value = expr.eval(&mut context).unwrap();
        assert_eq!(value.to_string(), "1");
    }
}

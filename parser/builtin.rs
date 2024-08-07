use std::rc::Rc;

use crate::{
    function::{DefinitionParameter, FunctionDefinition, FunctionSignature},
    lang_type::Type,
    module::ModuleDefinition,
    value::Value,
};

#[allow(dead_code)]
fn build_math_module() -> ModuleDefinition {
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

    module
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

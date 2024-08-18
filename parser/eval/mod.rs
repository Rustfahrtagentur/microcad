mod context;
mod error;
mod symbols;

pub use context::*;
pub use error::*;
pub use symbols::*;

use crate::language::{function::*, identifier::*, module::*, value::*};

pub trait Eval {
    type Output;

    /// Evaluate the type into an expression
    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error>;
}

// @todo Move this test elsewhere
#[test]
fn context_basic() {
    use crate::parser::*;

    let mut context = Context::default();

    context.add_symbol(Symbol::Value("a".into(), Value::Integer(1)));
    context.add_symbol(Symbol::Value("b".into(), Value::Integer(2)));

    assert_eq!(context.find_symbols(&"a".into())[0].name(), "a");
    assert_eq!(context.find_symbols(&"b".into())[0].name(), "b");

    let c = Parser::parse_rule_or_panic::<crate::language::assignment::Assignment>(
        Rule::assignment,
        "c = a + b",
    );

    c.eval(&mut context).unwrap();
}

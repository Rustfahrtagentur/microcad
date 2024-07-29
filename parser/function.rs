use crate::call::CallArgumentList;
use crate::expression::Expression;
use crate::identifier::Identifier;
use crate::langtype::Type;
use crate::parser::{Pair, Parse, ParseError, Rule};

struct VariableDeclaration {
    identifier: Identifier,
    ty: Option<Type>,
    default: Option<Expression>,
}

struct FunctionSignature {
    name: String,
    arguments: Vec<String>,
}

trait CallMethod {
    fn call_method(&self, lhs: Box<Expression>, name: &str, args: CallArgumentList);
}

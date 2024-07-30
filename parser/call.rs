use std::collections::BTreeMap;

use crate::expression::Expression;
use crate::identifier::{Identifier, IdentifierList, QualifiedName};
use crate::parser::*;

enum CallArgument {
    CallNamedArgument(Identifier, Box<Expression>),
    CallNamedTupleArgument(IdentifierList, Box<Expression>),
    CallPositionalArgument(Box<Expression>),
}

impl Parse for CallArgument {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::call_named_argument => {
                let mut pairs = pair.into_inner();
                let first = pairs.next().unwrap();
                let second = pairs.next().unwrap();

                Ok(CallArgument::CallNamedArgument(
                    Identifier::parse(first)?,
                    Box::new(Expression::parse(second)?),
                ))
            }
            Rule::call_named_tuple_argument => {
                let mut pairs = pair.into_inner();
                let first = pairs.next().unwrap();
                let second = pairs.next().unwrap();

                Ok(CallArgument::CallNamedTupleArgument(
                    IdentifierList::parse(first)?,
                    Box::new(Expression::parse(second)?),
                ))
            }
            Rule::expression => Ok(CallArgument::CallPositionalArgument(Box::new(
                Expression::parse(pair)?,
            ))),
            rule => unreachable!(
                "CallArgument::parse expected call argument, found {:?}",
                rule
            ),
        }
    }
}

#[derive(Default, Clone)]
pub struct CallArgumentList {
    positional: Vec<Box<Expression>>,
    named: BTreeMap<Identifier, Box<Expression>>,
}

impl CallArgumentList {
    fn get_named(&self, ident: &Identifier) -> Option<&Box<Expression>> {
        self.named.get(ident)
    }

    fn get_positional(&self, index: usize) -> Option<&Box<Expression>> {
        self.positional.get(index)
    }

    fn len(&self) -> usize {
        self.positional.len() + self.named.len()
    }

    fn is_empty(&self) -> bool {
        self.positional.is_empty() && self.named.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = &Box<Expression>> {
        self.positional.iter().chain(self.named.values())
    }

    fn insert_named(&mut self, ident: Identifier, expr: Box<Expression>) {
        self.named.insert(ident, expr);
    }

    fn insert_positional(&mut self, expr: Box<Expression>) {
        self.positional.push(expr);
    }
}

impl Parse for CallArgumentList {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut call_argument_list = CallArgumentList::default();

        match pair.as_rule() {
            Rule::call_argument_list => {
                for pair in pair.into_inner() {
                    match CallArgument::parse(pair)? {
                        CallArgument::CallNamedArgument(ident, expr) => {
                            call_argument_list.insert_named(ident, expr);
                        }
                        CallArgument::CallNamedTupleArgument(idents, expr) => {
                            for ident in idents {
                                call_argument_list.insert_named(ident, expr.clone());
                            }
                        }
                        CallArgument::CallPositionalArgument(expr) => {
                            call_argument_list.insert_positional(expr);
                        }
                    }
                }

                Ok(call_argument_list)
            }
            rule => unreachable!(
                "CallArgumentList::parse expected call argument list, found {:?}",
                rule
            ),
        }
    }
}

#[derive(Default, Clone)]
pub struct MethodCall {
    pub name: Identifier,
    pub argument_list: CallArgumentList,
}

impl Parse for MethodCall {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();
        println!("{:?}", pairs);

        Ok(MethodCall {
            name: Identifier::parse(pairs.next().unwrap())?,
            argument_list: if let Some(pair) = pairs.next() {
                CallArgumentList::parse(pair)?
            } else {
                CallArgumentList::default()
            },
        })
    }
}

struct Call {
    name: QualifiedName,
    argument_list: CallArgumentList,
}

impl Parse for Call {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut pairs = pair.into_inner();
        let (first, second) = (pairs.next().unwrap(), pairs.next().unwrap());

        Ok(Call {
            name: QualifiedName::parse(first)?,
            argument_list: CallArgumentList::parse(second)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call() {
        use pest::Parser;
        let pair = crate::parser::Parser::parse(Rule::call, "foo(1, 2, bar = 3, baz = 4)")
            .unwrap()
            .next()
            .unwrap();

        let call = Call::parse(pair).unwrap();

        assert_eq!(call.name, crate::identifier::QualifiedName::from("foo"));
        assert_eq!(call.argument_list.positional.len(), 2);
        assert_eq!(call.argument_list.named.len(), 2);
    }
}

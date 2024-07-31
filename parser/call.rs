use std::collections::BTreeMap;

use crate::expression::Expression;
use crate::identifier::{Identifier, IdentifierList, QualifiedName};
use crate::parser::*;

enum CallArgument {
    NamedArgument(Identifier, Box<Expression>),
    NamedTupleArgument(IdentifierList, Box<Expression>),
    PositionalArgument(Box<Expression>),
}

impl Parse for CallArgument {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::call_named_argument => {
                let mut pairs = pair.into_inner();
                let first = pairs.next().unwrap();
                let second = pairs.next().unwrap();

                Ok(CallArgument::NamedArgument(
                    Identifier::parse(first)?,
                    Box::new(Expression::parse(second)?),
                ))
            }
            Rule::call_named_tuple_argument => {
                let mut pairs = pair.into_inner();
                let first = pairs.next().unwrap();
                let second = pairs.next().unwrap();

                Ok(CallArgument::NamedTupleArgument(
                    IdentifierList::parse(first)?,
                    Box::new(Expression::parse(second)?),
                ))
            }
            Rule::expression => Ok(CallArgument::PositionalArgument(Box::new(
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
    positional: Vec<Expression>,
    named: BTreeMap<Identifier, Expression>,
}

impl CallArgumentList {
    pub fn get_named(&self) -> &BTreeMap<Identifier, Expression> {
        &self.named
    }

    pub fn get_named_arg(&self, ident: &Identifier) -> Option<&Expression> {
        self.named.get(ident)
    }

    pub fn get_positional(&self) -> &[Expression] {
        &self.positional
    }

    pub fn get_positional_arg(&self, index: usize) -> Option<&Expression> {
        self.positional.get(index)
    }

    pub fn len(&self) -> usize {
        self.positional.len() + self.named.len()
    }

    pub fn is_empty(&self) -> bool {
        self.positional.is_empty() && self.named.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Expression> {
        self.positional.iter().chain(self.named.values())
    }

    pub fn contains_positional(&self) -> bool {
        !self.positional.is_empty()
    }

    pub fn contains_named(&self) -> bool {
        !self.named.is_empty()
    }

    fn insert_named(&mut self, ident: Identifier, expr: Expression) -> Result<(), ParseError> {
        if self.named.contains_key(&ident) {
            return Err(ParseError::DuplicateNamedArgument(ident));
        }

        self.named.insert(ident, expr);
        Ok(())
    }

    fn insert_positional(&mut self, expr: Expression) -> Result<(), ParseError> {
        if !self.named.is_empty() {
            return Err(ParseError::PositionalArgumentAfterNamed);
        }
        self.positional.push(expr);
        Ok(())
    }
}

impl Parse for CallArgumentList {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut call_argument_list = CallArgumentList::default();

        match pair.as_rule() {
            Rule::call_argument_list => {
                for pair in pair.into_inner() {
                    match CallArgument::parse(pair)? {
                        CallArgument::NamedArgument(ident, expr) => {
                            call_argument_list.insert_named(ident, *expr)?;
                        }
                        CallArgument::NamedTupleArgument(idents, expr) => {
                            for ident in idents {
                                call_argument_list.insert_named(ident, *expr.clone())?;
                            }
                        }
                        CallArgument::PositionalArgument(expr) => {
                            call_argument_list.insert_positional(*expr)?;
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

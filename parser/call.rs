use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::eval::{Context, Symbol};
use crate::expression::Expression;
use crate::identifier::QualifiedName;
use crate::identifier::{Identifier, IdentifierList};

use crate::parser::*;

enum CallArgument {
    Named(Identifier, Box<Expression>),
    NamedTuple(IdentifierList, Box<Expression>),
    Position(Box<Expression>),
}

impl Parse for CallArgument {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        match pair.as_rule() {
            Rule::call_named_argument => {
                let mut pairs = pair.into_inner();
                let first = pairs.next().unwrap();
                let second = pairs.next().unwrap();

                Ok(CallArgument::Named(
                    Identifier::parse(first)?,
                    Box::new(Expression::parse(second)?),
                ))
            }
            Rule::call_named_tuple_argument => {
                let mut pairs = pair.into_inner();
                let first = pairs.next().unwrap();
                let second = pairs.next().unwrap();

                Ok(CallArgument::NamedTuple(
                    IdentifierList::parse(first)?,
                    Box::new(Expression::parse(second)?),
                ))
            }
            Rule::expression => Ok(CallArgument::Position(Box::new(Expression::parse(pair)?))),
            rule => unreachable!("CallArgument::parse expected call argument, found {rule:?}"),
        }
    }
}

#[derive(Default, Clone)]
pub struct PositionalNamedList<T> {
    positional: Vec<T>,
    named: BTreeMap<Identifier, T>,
}

pub type CallArgumentList = PositionalNamedList<Expression>;
pub type EvaluatedCallArgumentList = PositionalNamedList<crate::value::Value>;

impl<T> PositionalNamedList<T> {
    pub fn new() -> Self {
        Self {
            positional: Vec::new(),
            named: BTreeMap::new(),
        }
    }

    pub fn get_named(&self) -> &BTreeMap<Identifier, T> {
        &self.named
    }

    pub fn get_named_arg(&self, ident: &Identifier) -> Option<&T> {
        self.named.get(ident)
    }

    pub fn get_positional(&self) -> &[T] {
        &self.positional
    }

    pub fn get_positional_arg(&self, index: usize) -> Option<&T> {
        self.positional.get(index)
    }

    pub fn len(&self) -> usize {
        self.positional.len() + self.named.len()
    }

    pub fn is_empty(&self) -> bool {
        self.positional.is_empty() && self.named.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.positional.iter().chain(self.named.values())
    }

    pub fn contains_positional(&self) -> bool {
        !self.positional.is_empty()
    }

    pub fn contains_named(&self) -> bool {
        !self.named.is_empty()
    }

    fn insert_named(&mut self, ident: Identifier, v: T) -> Result<(), ParseError> {
        if self.named.contains_key(&ident) {
            return Err(ParseError::DuplicateNamedArgument(ident));
        }

        self.named.insert(ident, v);
        Ok(())
    }

    fn insert_positional(&mut self, v: T) -> Result<(), ParseError> {
        if !self.named.is_empty() {
            return Err(ParseError::PositionalArgumentAfterNamed);
        }
        self.positional.push(v);
        Ok(())
    }
}

impl CallArgumentList {
    pub fn eval(
        &self,
        context: &mut Context,
    ) -> Result<EvaluatedCallArgumentList, crate::eval::Error> {
        use crate::eval::Eval;
        let mut evaluated = EvaluatedCallArgumentList::new();
        for (ident, expr) in self.named.iter() {
            evaluated.insert_named(ident.clone(), expr.eval(context)?);
        }
        for expr in self.positional.iter() {
            evaluated.insert_positional(expr.eval(context)?);
        }
        Ok(evaluated)
    }
}

impl Parse for CallArgumentList {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        let mut call_argument_list = CallArgumentList::default();

        match pair.as_rule() {
            Rule::call_argument_list => {
                for pair in pair.into_inner() {
                    match CallArgument::parse(pair)? {
                        CallArgument::Named(ident, expr) => {
                            call_argument_list.insert_named(ident, *expr)?;
                        }
                        CallArgument::NamedTuple(idents, expr) => {
                            for ident in idents {
                                call_argument_list.insert_named(ident, *expr.clone())?;
                            }
                        }
                        CallArgument::Position(expr) => {
                            call_argument_list.insert_positional(*expr)?;
                        }
                    }
                }

                Ok(call_argument_list)
            }
            rule => {
                unreachable!("CallArgumentList::parse expected call argument list, found {rule:?}")
            }
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

#[derive(Default, Clone)]
pub struct Call {
    #[allow(dead_code)]
    name: QualifiedName,
    #[allow(dead_code)]
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

impl crate::eval::Eval for Call {
    fn eval(&self, context: &mut Context) -> Result<crate::value::Value, crate::eval::Error> {
        match context.resolve(&self.name)?.clone() {
            Symbol::Function(f) => {
                let args = self.argument_list.eval(context)?;
                {
                    f.call(args, context)
                }
            }
            _ => unimplemented!("Call::eval for symbol"),
        }
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

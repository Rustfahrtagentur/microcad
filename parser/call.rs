use std::collections::BTreeMap;

use crate::eval::{Context, Eval, Symbol};
use crate::expression::Expression;
use crate::identifier::QualifiedName;
use crate::identifier::{Identifier, IdentifierList};

use crate::{parser::*, with_pair_ok};

#[derive(Clone)]
enum CallArgument {
    Named(Identifier, Box<Expression>),
    NamedTuple(IdentifierList, Box<Expression>),
    Position(Box<Expression>),
}

impl Parse for CallArgument {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        match pair.as_rule() {
            Rule::call_named_argument => {
                let mut pairs = pair.into_inner();
                let first = pairs.next().unwrap();
                let second = pairs.next().unwrap();

                with_pair_ok!(
                    CallArgument::Named(
                        Identifier::parse(first)?.value().clone(),
                        Box::new(Expression::parse(second)?.value().clone()),
                    ),
                    p
                )
            }
            Rule::call_named_tuple_argument => {
                let mut pairs = pair.into_inner();
                let first = pairs.next().unwrap();
                let second = pairs.next().unwrap();

                with_pair_ok!(
                    CallArgument::NamedTuple(
                        IdentifierList::parse(first)?.value().clone(),
                        Box::new(Expression::parse(second)?.value().clone()),
                    ),
                    p
                )
            }
            Rule::expression => {
                with_pair_ok!(
                    CallArgument::Position(Box::new(Expression::parse(pair)?.value().clone())),
                    p
                )
            }
            rule => unreachable!("CallArgument::parse expected call argument, found {rule:?}"),
        }
    }
}

impl std::fmt::Display for CallArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CallArgument::Named(ident, expr) => write!(f, "{} = {}", ident, expr),
            CallArgument::NamedTuple(idents, expr) => {
                write!(f, "({}) = {}", idents, expr)
            }
            CallArgument::Position(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Clone, Debug, Default)]
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

impl<T> std::fmt::Display for PositionalNamedList<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut first = true;
        for arg in self.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        Ok(())
    }
}

impl Eval for CallArgumentList {
    type Output = EvaluatedCallArgumentList;

    fn eval(&self, context: &mut Context) -> Result<EvaluatedCallArgumentList, crate::eval::Error> {
        let mut evaluated = EvaluatedCallArgumentList::new();
        for expr in self.positional.iter() {
            evaluated.insert_positional(expr.eval(context)?).unwrap(); // Unwrap is safe because we checked for named arguments already
        }

        for (ident, expr) in self.named.iter() {
            evaluated
                .insert_named(ident.clone(), expr.eval(context)?)
                .unwrap(); // Unwrap is safe because we checked for duplicates in insert_named
        }
        Ok(evaluated)
    }
}

impl Parse for CallArgumentList {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut call_argument_list = CallArgumentList::default();
        let p = pair.clone();

        match pair.as_rule() {
            Rule::call_argument_list => {
                for pair in pair.into_inner() {
                    match CallArgument::parse(pair)?.value().clone() {
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

                with_pair_ok!(call_argument_list, p)
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
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        let mut pairs = pair.into_inner();
        println!("{:?}", pairs);

        with_pair_ok!(
            MethodCall {
                name: Identifier::parse(pairs.next().unwrap())?.value().clone(),
                argument_list: if let Some(pair) = pairs.next() {
                    CallArgumentList::parse(pair)?.value().clone()
                } else {
                    CallArgumentList::default()
                },
            },
            p
        )
    }
}

impl std::fmt::Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.argument_list)
    }
}

#[derive(Clone, Default)]
pub struct Call {
    #[allow(dead_code)]
    name: QualifiedName,
    #[allow(dead_code)]
    argument_list: CallArgumentList,
}

impl Parse for Call {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let p = pair.clone();
        Parser::ensure_rule(&pair, Rule::call);
        let mut pairs = pair.into_inner();

        let first = pairs.next().unwrap();

        with_pair_ok!(
            Call {
                name: QualifiedName::parse(first)?.value().clone(),
                argument_list: match pairs.next() {
                    Some(pair) => CallArgumentList::parse(pair)?.value().clone(),
                    None => CallArgumentList::default(),
                }
            },
            p
        )
    }
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.argument_list)
    }
}

impl crate::eval::Eval for Call {
    type Output = crate::value::Value;

    fn eval(&self, context: &mut Context) -> Result<crate::value::Value, crate::eval::Error> {
        match &self.name.eval(context)? {
            Symbol::Function(f) => {
                let args = self.argument_list.eval(context)?;
                f.call(args, context)
            }
            _ => unimplemented!("Call::eval for symbol"),
        }
    }
}

#[test]
fn call() {
    use pest::Parser;

    let pair = crate::parser::Parser::parse(Rule::call, "foo(1, 2, bar = 3, baz = 4)")
        .unwrap()
        .next()
        .unwrap();

    let call = Call::parse(pair).unwrap();

    assert_eq!(call.name, QualifiedName::from("foo"));
    assert_eq!(call.argument_list.positional.len(), 2);
    assert_eq!(call.argument_list.named.len(), 2);
}

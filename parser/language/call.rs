use super::{expression::*, identifier::*, value::*};
use crate::{eval::*, parser::*, with_pair_ok};

#[derive(Clone, Debug)]
struct CallArgument {
    name: Option<Identifier>,
    value: Expression,
}

impl Parse for CallArgument {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        match pair.clone().as_rule() {
            Rule::call_named_argument => {
                let mut inner = pair.clone().into_inner();
                let first = inner.next().unwrap();
                let second = inner.next().unwrap();

                with_pair_ok!(
                    CallArgument {
                        name: Some(Identifier::parse(first)?.value().clone()),
                        value: Expression::parse(second)?.value().clone(),
                    },
                    pair
                )
            }
            Rule::expression => {
                with_pair_ok!(
                    CallArgument {
                        name: None,
                        value: Expression::parse(pair.clone())?.value().clone(),
                    },
                    pair
                )
            }
            rule => unreachable!("CallArgument::parse expected call argument, found {rule:?}"),
        }
    }
}

impl std::fmt::Display for CallArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.name {
            Some(ref name) => write!(f, "{} = {}", name, self.value),
            None => write!(f, "{}", self.value),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PositionalNamedList<T> {
    positional: Vec<T>,
    named: std::collections::BTreeMap<Identifier, T>,
}

pub type CallArgumentList = PositionalNamedList<Expression>;
pub type EvaluatedCallArgumentList = PositionalNamedList<Value>;

impl<T> std::ops::Deref for PositionalNamedList<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.positional
    }
}

impl<T> PositionalNamedList<T> {
    pub fn new() -> Self {
        Self {
            positional: Vec::new(),
            named: std::collections::BTreeMap::new(),
        }
    }

    // Checks if the argument list contains a single argument only and returns it
    pub fn arg_1(&self, ident: &str) -> Result<&T, Error> {
        if self.len() != 1 {
            return Err(Error::ArgumentCountMismatch {
                expected: 1,
                found: self.len(),
            });
        }

        match self.get(&Identifier::from(ident), 0) {
            Some(v) => Ok(v),
            None => Err(Error::FunctionCallMissingArgument(Identifier::from(ident))),
        }
    }

    // Checks if the argument list contains a two argument only and returns them as tuple
    pub fn arg_2(&self, x: &str, y: &str) -> Result<(&T, &T), Error> {
        if self.len() != 2 {
            return Err(Error::ArgumentCountMismatch {
                expected: 2,
                found: self.len(),
            });
        }

        match (
            self.get(&Identifier::from(x), 0),
            self.get(&Identifier::from(y), 1),
        ) {
            (Some(x), Some(y)) => Ok((x, y)),
            (None, _) => Err(Error::FunctionCallMissingArgument(Identifier::from(x))),
            (_, None) => Err(Error::FunctionCallMissingArgument(Identifier::from(y))),
        }
    }

    /// Tries to get the argument by identifier, if it fails, it tries to get the argument by index
    pub fn get(&self, ident: &Identifier, index: usize) -> Option<&T> {
        match self.named.get(ident) {
            Some(v) => Some(v),
            None => self.positional.get(index),
        }
    }

    pub fn get_named(&self) -> &std::collections::BTreeMap<Identifier, T> {
        &self.named
    }

    pub fn get_named_arg(&self, ident: &Identifier) -> Option<&T> {
        self.named.get(ident)
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

    fn eval(&self, context: &mut Context) -> Result<EvaluatedCallArgumentList, Error> {
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

        match pair.clone().as_rule() {
            Rule::call_argument_list => {
                for pair in pair.clone().into_inner() {
                    let call = CallArgument::parse(pair.clone())?.value().clone();
                    match call.name {
                        Some(ident) => {
                            call_argument_list.insert_named(ident, call.value)?;
                        }
                        None => {
                            call_argument_list.insert_positional(call.value)?;
                        }
                    }
                }

                with_pair_ok!(call_argument_list, pair)
            }
            rule => {
                unreachable!("CallArgumentList::parse expected call argument list, found {rule:?}")
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MethodCall {
    pub name: Identifier,
    pub argument_list: CallArgumentList,
}

impl Parse for MethodCall {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut inner = pair.clone().into_inner();
        println!("{inner:?}");

        with_pair_ok!(
            MethodCall {
                name: Identifier::parse(inner.next().unwrap())?.value().clone(),
                argument_list: if let Some(pair) = inner.next() {
                    CallArgumentList::parse(pair)?.value().clone()
                } else {
                    CallArgumentList::default()
                },
            },
            pair
        )
    }
}

impl std::fmt::Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.argument_list)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Call {
    #[allow(dead_code)]
    pub name: QualifiedName,
    #[allow(dead_code)]
    pub argument_list: CallArgumentList,
}

impl Parse for Call {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        Parser::ensure_rule(&pair, Rule::call);
        let mut inner = pair.clone().into_inner();
        let first = inner.next().unwrap();

        with_pair_ok!(
            Call {
                name: QualifiedName::parse(first)?.value().clone(),
                argument_list: match inner.next() {
                    Some(pair) => CallArgumentList::parse(pair)?.value().clone(),
                    None => CallArgumentList::default(),
                }
            },
            pair
        )
    }
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.argument_list)
    }
}

impl Eval for Call {
    type Output = Option<Value>;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        let args = self.argument_list.eval(context)?;

        match &self.name.eval(context)? {
            Symbol::Function(f) => Ok(Some(f.call(args, context)?)),
            Symbol::BuiltinFunction(f) => Ok(Some(f.call(args, context)?)),
            Symbol::BuiltinModule(m) => Ok(None),
            _ => unimplemented!("Call::eval for symbol"),
        }
    }
}

#[test]
fn call() {
    use pest::Parser as _;
    let pair = Parser::parse(Rule::call, "foo(1, 2, bar = 3, baz = 4)")
        .unwrap()
        .next()
        .unwrap();

    let call = Call::parse(pair).unwrap();

    assert_eq!(call.name, QualifiedName::from("foo"));
    assert_eq!(call.argument_list.positional.len(), 2);
    assert_eq!(call.argument_list.named.len(), 2);
}

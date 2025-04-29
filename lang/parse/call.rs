use crate::{ord_map::*, parse::*, parser::*, syntax::*};

impl Parse for Call {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::call);
        let mut inner = pair.inner();
        let first = inner.next().expect("Expected qualified name");

        Ok(Call {
            name: QualifiedName::parse(first)?,
            argument_list: match inner.next() {
                Some(pair) => CallArgumentList::parse(pair)?,
                None => CallArgumentList::default(),
            },
            src_ref: pair.clone().into(),
        })
    }
}

impl Parse for CallArgumentList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut call_argument_list =
            CallArgumentList(Refer::new(OrdMap::default(), pair.clone().into()));

        match pair.as_rule() {
            Rule::call_argument_list => {
                for pair in pair.inner() {
                    call_argument_list
                        .push(CallArgument::parse(pair)?)
                        .map_err(ParseError::DuplicateCallArgument)?;
                }

                Ok(call_argument_list)
            }
            rule => {
                unreachable!("CallArgumentList::parse expected call argument list, found {rule:?}")
            }
        }
    }
}

impl Parse for CallArgument {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.clone().as_rule() {
            Rule::call_named_argument => {
                let mut inner = pair.inner();
                let first = inner.next().expect(INTERNAL_PARSE_ERROR);
                let second = inner.next().expect(INTERNAL_PARSE_ERROR);

                Ok(CallArgument {
                    name: Some(Identifier::parse(first)?),
                    value: Expression::parse(second)?,
                    src_ref: pair.src_ref(),
                })
            }
            Rule::expression => Ok(CallArgument {
                name: None,
                value: Expression::parse(pair.clone())?,
                src_ref: pair.into(),
            }),
            rule => unreachable!("CallArgument::parse expected call argument, found {rule:?}"),
        }
    }
}

impl Parse for MethodCall {
    fn parse(pair: Pair) -> ParseResult<Self> {
        let mut inner = pair.inner();

        Ok(MethodCall {
            name: Identifier::parse(inner.next().expect(INTERNAL_PARSE_ERROR))?,
            argument_list: if let Some(pair) = inner.next() {
                CallArgumentList::parse(pair)?
            } else {
                CallArgumentList::default()
            },
            src_ref: pair.clone().into(),
        })
    }
}

#[test]
fn call() {
    use crate::{parser::*, syntax::*};
    use pest::Parser as _;
    let pair = Pair::new(
        Parser::parse(Rule::call, "foo(1, 2, bar = 3, baz = 4)")
            .expect("test error")
            .next()
            .expect("test error"),
        0,
    );

    let call = Call::parse(pair).expect("test error");

    assert_eq!(call.name, "foo".into());
    assert_eq!(call.argument_list.len(), 4);

    // Count named arguments
    let named = call
        .argument_list
        .iter()
        .filter(|arg| arg.name.is_some())
        .count();
    assert_eq!(named, 2);
}

use crate::{eval::*, language::*, ord_map::OrdMap, parser::*, with_pair_ok};

#[derive(Clone, Debug, Default)]
pub struct CallArgumentList(OrdMap<Identifier, CallArgument>);

impl std::ops::Deref for CallArgumentList {
    type Target = OrdMap<Identifier, CallArgument>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CallArgumentList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Eval for CallArgumentList {
    type Output = CallArgumentValueList;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let mut call_argument_list = CallArgumentValueList::default();

        for arg in self.iter() {
            call_argument_list
                .push(arg.eval(context)?)
                .map_err(EvalError::DuplicateCallArgument)?;
        }

        Ok(call_argument_list)
    }
}

impl Parse for CallArgumentList {
    fn parse(pair: Pair<'_>) -> ParseResult<'_, Self> {
        let mut call_argument_list = CallArgumentList::default();

        match pair.clone().as_rule() {
            Rule::call_argument_list => {
                for pair in pair.clone().into_inner() {
                    call_argument_list
                        .push(CallArgument::parse(pair.clone())?.value)
                        .map_err(ParseError::DuplicateCallArgument)?;
                }
                with_pair_ok!(call_argument_list, pair)
            }
            rule => {
                unreachable!("CallArgumentList::parse expected call argument list, found {rule:?}")
            }
        }
    }
}

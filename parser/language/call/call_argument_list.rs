use crate::with_pair_ok;

use super::{
    CallArgument, CallArgumentValueList, Context, Error, Eval, Identifier, Pair, Parse,
    ParseResult, Rule,
};

#[derive(Clone, Debug, Default)]
pub struct CallArgumentList {
    arguments: Vec<CallArgument>,
    named: std::collections::HashMap<Identifier, usize>,
}

impl CallArgumentList {
    pub fn push(&mut self, arg: CallArgument) {
        self.arguments.push(arg.clone());
        if let Some(name) = arg.name {
            self.named.insert(name.clone(), self.arguments.len() - 1);
        }
    }

    pub fn get(&self, name: &Identifier) -> Option<&CallArgument> {
        self.named.get(name).map(|index| &self.arguments[*index])
    }
}

impl std::ops::Deref for CallArgumentList {
    type Target = Vec<CallArgument>;

    fn deref(&self) -> &Self::Target {
        &self.arguments
    }
}

impl Eval for CallArgumentList {
    type Output = CallArgumentValueList;

    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error> {
        let mut call_argument_list = CallArgumentValueList::default();

        for arg in self.iter() {
            call_argument_list.push(arg.eval(context)?);
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
                    call_argument_list.push(CallArgument::parse(pair.clone())?.value().clone());
                }
                with_pair_ok!(call_argument_list, pair)
            }
            rule => {
                unreachable!("CallArgumentList::parse expected call argument list, found {rule:?}")
            }
        }
    }
}

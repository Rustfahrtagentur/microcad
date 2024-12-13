// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD parser entities related to function or module calls

mod call_argument;
mod call_argument_list;
mod call_method;
mod method_call;
mod multiplicity;

pub use call_argument::*;
pub use call_argument_list::*;
pub use method_call::*;
pub use multiplicity::*;

use crate::{eval::*, parse::*, parse::*, parser::*, src_ref::*};

/// trait for calls of modules or functions with argument list
pub trait CallTrait {
    type Output;

    /// Evaluate call into value (if possible)
    fn call(&self, args: &CallArgumentList, context: &mut Context) -> Result<Self::Output>;
}

/// Call of a function or module initialization
#[derive(Clone, Debug, Default)]
pub struct Call {
    /// Qualified name of the call
    pub name: QualifiedName,
    /// Argument list of the call
    pub argument_list: CallArgumentList,
    /// Source code reference
    src_ref: SrcRef,
}

impl SrcReferrer for Call {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Sym for Call {
    fn id(&self) -> Option<microcad_core::Id> {
        self.name.id()
    }
}

impl Parse for Call {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::call);
        let mut inner = pair.inner();
        let first = inner.next().unwrap();

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

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({:?})", self.name, self.argument_list)
    }
}

pub enum CallResult {
    Nodes(Vec<crate::ObjectNode>),
    Value(crate::eval::Value),
    None,
}

impl Eval for Call {
    type Output = CallResult;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        match self.name.eval(context)? {
            Symbol::Function(f) => match f.call(&self.argument_list, context)? {
                Some(value) => Ok(CallResult::Value(value)),
                None => Ok(CallResult::None),
            },
            Symbol::BuiltinFunction(f) => match f.call(&self.argument_list, context)? {
                Some(value) => Ok(CallResult::Value(value)),
                None => Ok(CallResult::None),
            },
            Symbol::BuiltinModule(m) => {
                Ok(CallResult::Nodes(m.call(&self.argument_list, context)?))
            }
            Symbol::Module(m) => Ok(CallResult::Nodes(m.call(&self.argument_list, context)?)),
            Symbol::Invalid => {
                // We don't do anything if the symbol is not found, because an error has been already raised before
                Ok(CallResult::None)
            }
            symbol => {
                use crate::diag::PushDiag;
                use anyhow::anyhow;
                context.error(self, anyhow!("{} is not callable", symbol))?;
                Ok(CallResult::None)
            }
        }
    }
}

#[test]
fn call() {
    use pest::Parser as _;
    let pair = Pair::new(
        Parser::parse(Rule::call, "foo(1, 2, bar = 3, baz = 4)")
            .unwrap()
            .next()
            .unwrap(),
        0,
    );

    let call = Call::parse(pair).unwrap();

    assert_eq!(call.name, QualifiedName::from("foo"));
    assert_eq!(call.argument_list.len(), 4);

    // Count named arguments
    let named = call
        .argument_list
        .iter()
        .filter(|arg| arg.name.is_some())
        .count();
    assert_eq!(named, 2);
}

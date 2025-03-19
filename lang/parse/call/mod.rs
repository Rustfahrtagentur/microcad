// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad parser entities related to function or module calls

mod call_argument;
mod call_argument_list;
mod method_call;

pub use call_argument::*;
pub use call_argument_list::*;
pub use method_call::*;

use crate::{objects::*, parse::*, parser::*, src_ref::*};

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

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({})", self.name, self.argument_list)
    }
}

impl PrintSyntax for Call {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}Call '{}':", "", self.name)?;
        self.argument_list
            .iter()
            .try_for_each(|a| a.print_syntax(f, depth + 1))
    }
}

/// Result of a call
pub enum CallResult {
    /// Call returned nodes
    Nodes(Vec<ObjectNode>),

    /// Call returned a single value
    Value(crate::Value),

    /// Call returned nothing
    None,
}

#[test]
fn call() {
    use pest::Parser as _;
    let pair = Pair::new(
        Parser::parse(Rule::call, "foo(1, 2, bar = 3, baz = 4)")
            .expect("test error")
            .next()
            .expect("test error"),
        0,
    );

    let call = Call::parse(pair).expect("test error");

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

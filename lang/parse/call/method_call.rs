// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Method call

use crate::{eval::*, parse::*, parser::*, src_ref::*};

/// Method call
#[derive(Clone, Debug)]
pub struct MethodCall {
    /// Name of the method
    pub name: Identifier,
    /// List of arguments
    pub argument_list: CallArgumentList,
    /// Source code reference
    src_ref: SrcRef,
}

impl MethodCall {
    /// Evaluate the method call in a context
    pub fn eval(&self, context: &mut Context, lhs: &Expression) -> EvalResult<Value> {
        use call::call_method::CallMethod;

        match lhs.eval(context)? {
            Value::Node(node) => node.call_method(&self.name, &self.argument_list, self.src_ref()),
            Value::List(list) => list.call_method(&self.name, &self.argument_list, self.src_ref()),
            _ => {
                context
                    .error_with_stack_trace(self, EvalError::UnknownMethod(self.name.clone()))?;
                Ok(Value::Invalid)
            }
        }
    }
}

impl SrcReferrer for MethodCall {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
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

impl std::fmt::Display for MethodCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}({:?})", self.name, self.argument_list)
    }
}

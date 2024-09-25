// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Method call

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

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
    pub fn eval(&self, context: &mut Context, lhs: &Box<Expression>) -> Result<Value> {
        let name: &str = &self.name.to_string();
        let args = self.argument_list.eval(context)?;

        use call::call_method::CallMethod;

        match lhs.eval(context)? {
            Value::Node(node) => node.call_method(&self.name, &args, self.src_ref()),
            Value::List(list) => match name {
                "len" => Ok(Value::Integer(Refer::new(
                    list.len() as i64,
                    list.src_ref(),
                ))),
                _ => Err(EvalError::UnknownMethod(name.into())),
            },
            _ => Err(EvalError::UnknownMethod(name.into())),
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
            name: Identifier::parse(inner.next().unwrap())?,
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

// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, resolve::*};

/// A stack with a list of stack frames.
#[derive(Default)]
pub struct Stack(Vec<StackFrame>);

impl Stack {
    /// Add a new variable to current stack frame.
    pub fn add(&mut self, id: Option<Identifier>, frame: Symbol) -> EvalResult<()> {
        let id = if let Some(id) = id { id } else { frame.id() };
        let name = frame.full_name();
        if name.is_qualified() {
            log::debug!("Adding {name} as {id} to local stack");
        } else {
            log::debug!("Adding {id} to local stack");
        }

        match self.0.last_mut() {
            Some(StackFrame::Source(_, last)) | Some(StackFrame::Body(last)) => {
                last.insert(id.clone(), frame);
                log::trace!("Local Stack:\n{self}");
                Ok(())
            }
            _ => Err(EvalError::LocalStackEmpty(id)),
        }
    }

    /// Get name of current namespace.
    pub fn current_namespace(&self) -> QualifiedName {
        QualifiedName(self.0.iter().filter_map(|locals| locals.id()).collect())
    }


    /// Pretty print call trace.
    pub fn pretty_print_call_trace(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl super::GetSourceByHash,
    ) -> std::fmt::Result {
        for (idx, call_stack_frame) in self.0.iter().filter_map(|frame| match frame {
            StackFrame::Call(call) => Some(call),
            _ => None 
        }).enumerate() {
            call_stack_frame.pretty_print(f, source_by_hash, idx)?;
        }
        Ok(())
    }

}

impl Locals for Stack {
    fn open_source(&mut self, id: Identifier) {
        self.0.push(StackFrame::Source(id, SymbolMap::new()));
    }

    fn open_body(&mut self) {
        self.0.push(StackFrame::Body(SymbolMap::new()));
    }

    fn open_namespace(&mut self, id: Identifier) {
        self.0.push(StackFrame::Namespace(id));
    }

    fn open_call(&mut self, symbol: Symbol, args: CallArgumentList, src_ref: impl SrcReferrer) {
        self.0.push(StackFrame::Call(CallStackFrame::new(symbol, args, src_ref)))
    }

    fn close(&mut self) {
        self.0.pop();
    }

    fn add_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()> {
        self.add(Some(id.clone()), Symbol::new_constant(id, value))
    }

    fn fetch(&self, id: &Identifier) -> EvalResult<Symbol> {
        // search from inner scope to root scope to shadow outside locals
        for frame in self.0.iter().rev() {
            match frame {
                StackFrame::Source(_, locals) | StackFrame::Body(locals) => {
                    if let Some(local) = locals.get(id) {
                        log::debug!("Fetched {id} from locals");
                        return Ok(local.clone());
                    }
                }
                _ => (),
            }
        }
        Err(EvalError::LocalNotFound(id.clone()))
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            writeln!(f, "<empty stack>")
        } else {
            for (n, locals) in self.0.iter().enumerate() {
                locals.print(f, n * 4)?;
            }
            Ok(())
        }
    }
}

#[test]
#[allow(clippy::unwrap_used)]
fn local_stack() {
    use std::ops::Deref;

    let mut stack = Stack::default();

    let make_int =
        |id, value| Symbol::new_constant(id, Value::Integer(Refer::new(value, SrcRef(None))));

    let fetch_int = |stack: &Stack, id: &str| -> Option<i64> {
        match stack.fetch(&id.into()) {
            Ok(node) => match &node.borrow().def {
                SymbolDefinition::Constant(_, Value::Integer(value)) => Some(*value.deref()),
                _ => todo!("error"),
            },
            _ => None,
        }
    };

    stack.open_source("test".into());
    assert!(stack.current_namespace() == "test".into());

    assert!(stack.add(None, make_int("a".into(), 1)).is_ok());

    println!("{stack}");

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.open_body();
    assert!(stack.current_namespace() == "test".into());

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    assert!(stack.add(None, make_int("b".into(), 2)).is_ok());

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").unwrap() == 2);
    assert!(fetch_int(&stack, "c").is_none());

    // test alias
    assert!(stack.add(Some("x".into()), make_int("x".into(), 3)).is_ok());

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").unwrap() == 2);
    assert!(fetch_int(&stack, "x").unwrap() == 3);

    stack.close();
    assert!(stack.current_namespace() == "test".into());

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.close();
    assert!(stack.current_namespace().is_empty());
}

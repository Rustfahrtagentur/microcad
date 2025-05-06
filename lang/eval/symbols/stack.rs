// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, resolve::*};

/// A stack with a list of stack frames.
#[derive(Default)]
pub struct Stack(Vec<StackFrame>);

impl Stack {
    /// Put (or overwrite any existing) a *symbol* into the current stack frame.
    /// - `id`: *identifier* of the symbol to add/set. The *symbol's* internal *identifier* is used when `None`.
    pub fn put_local(&mut self, id: Option<Identifier>, symbol: Symbol) -> EvalResult<()> {
        let id = if let Some(id) = id { id } else { symbol.id() };
        let name = symbol.full_name();
        match self.0.last_mut() {
            Some(StackFrame::Source(_, last)) | Some(StackFrame::Body(last)) => {
                let op = if last.insert(id.clone(), symbol).is_some() {
                    "Added"
                } else {
                    "Set"
                };
                if name.is_qualified() {
                    log::debug!("{op} {name} as {id} to local stack");
                } else {
                    log::debug!("{op} {id} to local stack");
                }

                log::trace!("Local Stack:\n{self}");
                Ok(())
            }
            Some(StackFrame::Namespace(_, _))
            | Some(StackFrame::Call {
                symbol: _,
                args: _,
                src_ref: _,
            }) => Err(EvalError::WrongStackFrame(id, "call frame")),
            _ => Err(EvalError::LocalStackEmpty(id)),
        }
    }

    /// Get name of current namespace.
    pub fn current_namespace(&self) -> QualifiedName {
        QualifiedName(self.0.iter().filter_map(|locals| locals.id()).collect())
    }

    /// Return the current *stack frame* if there is any.
    pub fn current_frame(&self) -> Option<&StackFrame> {
        self.0.last()
    }

    /// Pretty print call trace.
    pub fn pretty_print_call_trace(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl super::GetSourceByHash,
    ) -> std::fmt::Result {
        for (idx, frame) in self
            .0
            .iter()
            .filter(|frame| {
                matches!(
                    frame,
                    StackFrame::Call {
                        symbol: _,
                        args: _,
                        src_ref: _
                    }
                )
            })
            .enumerate()
        {
            frame.pretty_print(f, source_by_hash, idx)?;
        }
        Ok(())
    }
}

impl Locals for Stack {
    fn open(&mut self, frame: StackFrame) {
        self.0.push(frame);
    }

    fn close(&mut self) {
        self.0.pop();
    }

    fn set_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()> {
        match &self.current_frame() {
            Some(StackFrame::Namespace(_, _)) => Err(EvalError::NoVariablesAllowedIn("namespaces")),
            _ => self.put_local(Some(id.clone()), Symbol::new_constant(id, value)),
        }
    }

    fn get_local_value(&mut self, id: &Identifier) -> EvalResult<Value> {
        match self.fetch(id) {
            Ok(symbol) => match &symbol.borrow().def {
                SymbolDefinition::Constant(_, value) => Ok(value.clone()),
                _ => Err(EvalError::LocalNotFound(id.clone())),
            },
            Err(_) => Err(EvalError::LocalNotFound(id.clone())),
        }
    }

    fn fetch(&self, id: &Identifier) -> EvalResult<Symbol> {
        // search from inner scope to root scope to shadow outside locals
        for (pos, frame) in self.0.iter().rev().enumerate() {
            match frame {
                StackFrame::Source(_, locals)
                | StackFrame::Body(locals)
                | StackFrame::Module(_, locals)
                | StackFrame::ModuleInit(locals) => {
                    if let Some(local) = locals.get(id) {
                        log::debug!("Fetched {id} from locals");
                        return Ok(local.clone());
                    }
                }
                // stop stack lookup at calls
                StackFrame::Namespace(_, _) => {
                    log::trace!("stop at call frame");
                    break;
                }
                StackFrame::Call {
                    symbol: _,
                    args: _,
                    src_ref: _,
                } => {
                    if pos > 0 {
                        log::trace!("stop at call frame");
                        break;
                    }
                }
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

    assert!(stack.put_local(None, make_int("a".into(), 1)).is_ok());

    println!("{stack}");

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.open_body();
    assert!(stack.current_namespace() == "test".into());

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    assert!(stack.put_local(None, make_int("b".into(), 2)).is_ok());

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").unwrap() == 2);
    assert!(fetch_int(&stack, "c").is_none());

    // test alias
    assert!(
        stack
            .put_local(Some("x".into()), make_int("x".into(), 3))
            .is_ok()
    );

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

// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, resolve::*};
use std::collections::BTreeMap;

/// A stack with a list of local variables for each stack frame.
#[derive(Default)]
pub struct LocalStack(Vec<LocalFrame>);

impl LocalStack {
    /// Add a new variable to current stack frame.
    pub fn add(&mut self, id: Option<Identifier>, frame: SymbolNodeRcMut) -> EvalResult<()> {
        let id = if let Some(id) = id {
            id
        } else {
            frame.borrow().id()
        };
        let name = frame.borrow().full_name();
        if name.is_qualified() {
            log::debug!("Adding {name} as {id} to local stack");
        } else {
            log::debug!("Adding {id} to local stack");
        }

        match self.0.last_mut() {
            Some(LocalFrame::Source(_, last)) | Some(LocalFrame::Scope(last)) => {
                last.insert(id.clone(), frame);
                log::trace!("Local Stack:\n{self}");
                Ok(())
            }
            _ => Err(EvalError::NoLocalStack(id)),
        }
    }

    /// get name of current namespace
    pub fn current_namespace(&self) -> QualifiedName {
        QualifiedName(self.0.iter().filter_map(|locals| locals.id()).collect())
    }
}

impl Locals for LocalStack {
    /// Open a new source (stack push).
    fn open_source(&mut self, id: Identifier) {
        self.0.push(LocalFrame::Source(id, BTreeMap::new()));
    }

    /// Open a new scope (stack push).
    fn open_scope(&mut self) {
        self.0.push(LocalFrame::Scope(BTreeMap::new()));
    }

    /// Open a new scope (stack push).
    fn open_namespace(&mut self, id: Identifier) {
        self.0.push(LocalFrame::Namespace(id));
    }

    /// Open a new scope (stack push).
    fn open_module(&mut self, id: Identifier) {
        self.0.push(LocalFrame::Module(id));
    }

    /// Close scope (stack pop).
    fn close(&mut self) {
        self.0.pop();
    }

    fn add_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()> {
        self.add(Some(id.clone()), SymbolNode::new_constant(id, value))
    }

    /// Fetch a local variable from current stack frame.
    fn fetch(&self, id: &Identifier) -> EvalResult<SymbolNodeRcMut> {
        // search from inner scope to root scope to shadow outside locals
        for frame in self.0.iter().rev() {
            match frame {
                LocalFrame::Source(_, locals) | LocalFrame::Scope(locals) => {
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

impl std::fmt::Display for LocalStack {
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

    crate::env_logger_init();

    let mut stack = LocalStack::default();

    let make_int =
        |id, value| SymbolNode::new_constant(id, Value::Integer(Refer::new(value, SrcRef(None))));

    let fetch_int = |stack: &LocalStack, id: &str| -> Option<i64> {
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

    stack.open_scope();
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

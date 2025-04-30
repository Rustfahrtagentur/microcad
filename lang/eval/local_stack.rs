// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, resolve::*};
use log::debug;
use std::collections::BTreeMap;

/// A stack frame is map of local variables.
enum Locals {
    Source(Identifier, BTreeMap<Identifier, SymbolNodeRcMut>),
    Namespace(Identifier),
    Module(Identifier),
    Scope(BTreeMap<Identifier, SymbolNodeRcMut>),
}

impl Locals {
    /// Get identifier if available or panic.
    pub fn id(&self) -> Option<Identifier> {
        match self {
            Locals::Source(id, _) | Locals::Namespace(id) | Locals::Module(id) => Some(id.clone()),
            _ => None,
        }
    }

    fn print(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        let (map, depth) = match self {
            Locals::Source(id, map) => {
                writeln!(f, "{:depth$}{id} (source):", "")?;
                (map, depth + 2)
            }
            Locals::Namespace(id) => return write!(f, "{:depth$}{id} (namespace)", ""),
            Locals::Module(id) => return write!(f, "{:depth$}{id} (module)", ""),
            Locals::Scope(map) => (map, depth),
        };

        for (id, local) in map.iter() {
            writeln!(f, "{:depth$}{id} [{}]", "", local.borrow().full_name())?
        }

        Ok(())
    }
}

/// A stack with a list of local variables for each stack frame.
#[derive(Default)]
pub struct LocalStack(Vec<Locals>);

impl LocalStack {
    /// Open a new source (stack push).
    pub fn open_source(&mut self, id: Identifier) {
        self.0.push(Locals::Source(id, BTreeMap::new()));
    }

    /// Open a new scope (stack push).
    pub fn open_scope(&mut self) {
        self.0.push(Locals::Scope(BTreeMap::new()));
    }

    /// Open a new scope (stack push).
    pub fn open_namespace(&mut self, id: Identifier) {
        self.0.push(Locals::Namespace(id));
    }

    /// Open a new scope (stack push).
    pub fn open_module(&mut self, id: Identifier) {
        self.0.push(Locals::Module(id));
    }

    /// Close scope (stack pop).
    pub fn close(&mut self) {
        self.0.pop();
    }

    /// Add a new variable to current stack frame.
    pub fn add(&mut self, id: Option<Identifier>, local: SymbolNodeRcMut) -> EvalResult<()> {
        let id = if let Some(id) = id {
            id
        } else {
            local.borrow().id()
        };
        let name = local.borrow().full_name();
        if name.is_qualified() {
            debug!("Adding {name} as {id} to local stack");
        } else {
            debug!("Adding {id} to local stack");
        }

        match self.0.last_mut() {
            Some(Locals::Source(_, last)) | Some(Locals::Scope(last)) => {
                last.insert(id.clone(), local);
                Ok(())
            }
            _ => Err(EvalError::NoLocalStack(id)),
        }
    }

    /// Fetch a local variable from current stack frame.
    pub fn fetch(&self, id: &Identifier) -> EvalResult<SymbolNodeRcMut> {
        // search from inner scope to root scope to shadow outside locals
        for locals in self.0.iter().rev() {
            match locals {
                Locals::Source(_, locals) | Locals::Scope(locals) => {
                    if let Some(local) = locals.get(id) {
                        debug!("Fetched {id} from locals");
                        return Ok(local.clone());
                    }
                }
                _ => (),
            }
        }
        Err(super::EvalError::LocalNotFound(id.clone()))
    }

    pub fn get_name(&self) -> QualifiedName {
        QualifiedName(self.0.iter().filter_map(|locals| locals.id()).collect())
    }
}

impl std::fmt::Display for LocalStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            writeln!(f, "<empty stack>")
        } else {
            for (n, locals) in self.0.iter().enumerate() {
                locals.print(f, n)?;
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
    assert!(stack.get_name() == "test".into());

    assert!(stack.add(None, make_int("a".into(), 1)).is_ok());

    println!("{stack}");

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.open_scope();
    assert!(stack.get_name() == "test".into());

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
    assert!(stack.get_name() == "test".into());

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.close();
    assert!(stack.get_name().is_empty());
}

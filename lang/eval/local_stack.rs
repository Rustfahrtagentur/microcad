// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, resolve::*, Id};
use log::debug;

/// A stack frame is map of local variables
#[derive(Default)]
struct Locals(std::collections::BTreeMap<Id, SymbolNodeRcMut>);

impl Locals {
    fn print(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        for (id, local) in self.iter() {
            match local.borrow().full_name() {
                Some(name) => writeln!(f, "{:depth$} {id} [{name}]", "")?,
                None => writeln!(f, "{:depth$} {id}", "")?,
            }
        }
        Ok(())
    }
}

impl std::ops::Deref for Locals {
    type Target = std::collections::BTreeMap<Id, SymbolNodeRcMut>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Locals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A stack with a list of local variables for each stack frame
#[derive(Default)]
pub struct LocalStack(Vec<Locals>);

impl LocalStack {
    /// Open a new scope (stack push)
    pub fn open_scope(&mut self) {
        self.0.push(Default::default());
    }

    /// Close scope (stack pop)
    pub fn close_scope(&mut self) {
        self.0.pop();
    }

    /// Add a new variable to current stack frame
    pub fn add(&mut self, local: SymbolNodeRcMut) {
        let id = local.borrow().id();
        if let Some(name) = local.borrow().full_name() {
            if name.is_qualified() {
                debug!("Adding {name} as {id} to local stack");
            } else {
                debug!("Adding {id} to local stack");
            }
        }
        if let Some(last) = self.0.last_mut() {
            last
        } else {
            self.0.push(Locals::default());
            self.0
                .last_mut()
                .expect("cannot push symbol onto an empty local stack")
        }
        .insert(id, local);
    }

    /// Fetch a local variable from current stack frame
    pub fn fetch(&self, id: &Id) -> EvalResult<SymbolNodeRcMut> {
        debug!("Fetching {id} from locals");
        // search from inner scope to root scope to shadow outside locals
        for map in self.0.iter().rev() {
            if let Some(local) = map.get(id) {
                return Ok(local.clone());
            }
        }
        Err(super::EvalError::LocalNotFound(id.clone()))
    }
}

impl std::fmt::Display for LocalStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (n, locals) in self.0.iter().enumerate() {
            locals.print(f, n)?;
        }
        Ok(())
    }
}

#[test]
#[allow(clippy::unwrap_used)]
fn local_stack() {
    use std::ops::Deref;

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

    stack.add(make_int("a".into(), 1));
    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.open_scope();

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.add(make_int("b".into(), 2));

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").unwrap() == 2);
    assert!(fetch_int(&stack, "c").is_none());

    stack.add(make_int("c".into(), 3));

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").unwrap() == 2);
    assert!(fetch_int(&stack, "c").unwrap() == 3);

    stack.close_scope();

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());
}

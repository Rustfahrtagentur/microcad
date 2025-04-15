// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, resolve::SymbolNodeRcMut, syntax::*, value::*, Id};
use log::debug;
use std::collections::BTreeMap;

/// A local variable
pub enum LocalDefinition {
    Value(Value),
    Expression(Expression),
    Symbol(SymbolNodeRcMut),
}

/// A stack frame is map of local variables
type Locals = BTreeMap<Id, LocalDefinition>;

/// A stack with a list of local variables for each stack frame
pub struct LocalStack(Vec<Locals>);

impl Default for LocalStack {
    fn default() -> Self {
        Self(vec![BTreeMap::new()])
    }
}

impl LocalStack {
    /// Open a new scope (stack push)
    pub fn open_scope(&mut self) {
        self.0.push(BTreeMap::new());
    }

    /// Close scope (stack pop)
    pub fn close_scope(&mut self) {
        self.0.pop();
    }

    /// Add a new variable to current stack frame
    pub fn add(&mut self, id: Id, local: LocalDefinition) {
        self.0
            .last_mut()
            .expect("cannot push symbol onto an empty local stack")
            .insert(id, local);
    }

    /// Fetch a local variable from current stack frame
    pub fn fetch<'a>(&'a self, id: &Id) -> EvalResult<&'a LocalDefinition> {
        debug!("fetching  {id} in locals");
        for map in self.0.iter().rev() {
            if let Some(local) = map.get(id) {
                return Ok(local);
            }
        }
        Err(super::EvalError::LocalNotFound(id.clone()))
    }
}

#[test]
#[allow(clippy::unwrap_used)]
fn local_stack() {
    use crate::src_ref::{Refer, SrcRef};
    let mut stack = LocalStack::default();

    let make_int = |value| LocalDefinition::Value(Value::Integer(Refer::new(value, SrcRef(None))));

    let fetch_int = |stack: &LocalStack, id: &str| -> Option<i64> {
        match stack.fetch(&id.into()) {
            Ok(LocalDefinition::Value(Value::Integer(i))) => Some(i.value),
            _ => None,
        }
    };

    stack.add("a".into(), make_int(1));
    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.open_scope();

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.add("b".into(), make_int(2));

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").unwrap() == 2);
    assert!(fetch_int(&stack, "c").is_none());

    stack.add("c".into(), make_int(3));

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").unwrap() == 2);
    assert!(fetch_int(&stack, "c").unwrap() == 3);

    stack.close_scope();

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());
}

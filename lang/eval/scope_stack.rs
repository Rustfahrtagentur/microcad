// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{syntax::*, value::*, Id};
use std::collections::BTreeMap;

/// A local variable
pub enum LocalDefinition {
    Value(Value),
    Expression(Expression),
}

/// A stack frame is map of local variables
type StackFrame = BTreeMap<Id, LocalDefinition>;

/// A stack with a list of local variables for each stack frame
pub struct ScopeStack(Vec<StackFrame>);

impl Default for ScopeStack {
    fn default() -> Self {
        Self(vec![BTreeMap::new()])
    }
}

impl ScopeStack {
    /// Open a new scope (stack push)
    pub fn open_scope(&mut self) {
        self.0.push(BTreeMap::new());
    }

    /// Close scope (stack pop)
    pub fn close_scope(&mut self) {
        self.0.pop();
    }

    /// Add a new variable to current stack frame
    pub fn add(&mut self, id: Id, symbol: LocalDefinition) {
        self.0
            .last_mut()
            .expect("cannot push symbol onto empty scope stack")
            .insert(id, symbol);
    }

    /// Fetch a local variable from current stack frame
    pub fn fetch<'a>(&'a self, id: &Id) -> Option<&'a LocalDefinition> {
        for map in self.0.iter().rev() {
            if let Some(symbol) = map.get(id) {
                return Some(symbol);
            }
        }
        None
    }
}

#[test]
#[allow(clippy::unwrap_used)]
fn scope_stack() {
    use crate::src_ref::{Refer, SrcRef};
    let mut stack = ScopeStack::default();

    let make_int = |value| LocalDefinition::Value(Value::Integer(Refer::new(value, SrcRef(None))));

    let fetch_int = |stack: &ScopeStack, id: &str| -> Option<i64> {
        stack.fetch(&id.into()).and_then(|v| match v {
            LocalDefinition::Value(Value::Integer(i)) => Some(i.value),
            _ => None,
        })
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

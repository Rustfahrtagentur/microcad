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

impl FullyQualify for LocalDefinition {
    fn full_name(&self) -> Option<QualifiedName> {
        match self {
            LocalDefinition::Value(_) | LocalDefinition::Expression(_) => None,
            LocalDefinition::Symbol(rc_mut) => rc_mut.borrow().full_name(),
        }
    }
}

impl SrcReferrer for LocalDefinition {
    fn src_ref(&self) -> SrcRef {
        match self {
            LocalDefinition::Value(value) => value.src_ref(),
            LocalDefinition::Expression(expression) => expression.src_ref(),
            LocalDefinition::Symbol(node) => node.borrow().src_ref(),
        }
    }
}

/// A stack frame is map of local variables
#[derive(Default)]
struct Locals(BTreeMap<Id, LocalDefinition>);

impl Locals {
    fn print(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        for (id, def) in self.iter() {
            match def.full_name() {
                Some(name) => writeln!(f, "{:depth$} {id} [{name}]", "")?,
                None => writeln!(f, "{:depth$} {id}", "")?,
            }
        }
        Ok(())
    }
}

impl std::ops::Deref for Locals {
    type Target = BTreeMap<Id, LocalDefinition>;

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
    pub fn add(&mut self, id: Id, local: LocalDefinition) {
        if let Some(name) = local.full_name() {
            debug!("Adding {name} as {id} to local stack");
        } else {
            debug!("Adding {id} to local stack");
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
    pub fn fetch<'a>(&'a self, id: &Id) -> EvalResult<&'a LocalDefinition> {
        debug!("Fetching {id} from locals");
        for map in self.0.iter().rev() {
            if let Some(local) = map.get(id) {
                return Ok(local);
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

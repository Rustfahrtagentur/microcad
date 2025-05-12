// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, resolve::*};

/// A stack with a list of stack frames.
///
/// [`StackFrame`]s can have the following different types:
/// - source file (bottom of stack)
/// - namespaces ( e.g. `namespace my_lib { ... }`)
/// - module calls (e.g. `std::geo2d::circle(radius = 1m)`)
/// - function calls (e.g. `std::print("µcad")`)
/// - bodies (e.g. `{ ... }`)
#[derive(Default)]
pub struct Stack(Vec<StackFrame>);

impl Stack {
    /// Put (or overwrite any existing) *symbol* into the current stack frame.
    /// - `id`: *identifier* of the symbol to add/set. The *symbol's* internal *identifier* is used when `None`.
    pub fn put_local(&mut self, id: Option<Identifier>, symbol: Symbol) -> EvalResult<()> {
        let id = if let Some(id) = id { id } else { symbol.id() };
        let name = symbol.full_name();
        for (pos, frame) in self.0.iter_mut().rev().enumerate() {
            match frame {
                StackFrame::Source(_, last)
                | StackFrame::Body(last)
                | StackFrame::Module(_, last)
                | StackFrame::ModuleInit(last) => {
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
                    return Ok(());
                }
                // RULE: no locals available on namespace frame
                StackFrame::Namespace(_, _) => {
                    return Err(EvalError::WrongStackFrame(id, "namespace"))
                }
                StackFrame::Call {
                    symbol: _,
                    args: _,
                    src_ref: _,
                } => {
                    // RULE: top call frame is transparent on stack
                    if pos > 0 {
                        return Err(EvalError::WrongStackFrame(id, "call"));
                    }
                }
            }
        }
        Err(EvalError::LocalStackEmpty(id))
    }

    /// Return most top stack frame of type module
    fn current_module_id(&self) -> Option<&Identifier> {
        self.0.iter().rev().find_map(|frame| {
            if let StackFrame::Module(id, _) = frame {
                Some(id)
            } else {
                None
            }
        })
    }

    /// Get name of current namespace.
    pub fn current_namespace(&self) -> QualifiedName {
        if self.0.len() > 1 {
            QualifiedName::no_ref(
                self.0[1..]
                    .iter()
                    .filter_map(|locals| locals.id())
                    .collect(),
            )
        } else {
            QualifiedName::default()
        }
    }

    /// Get name of current namespace.
    pub fn current_module(&self) -> Option<QualifiedName> {
        if let Some(id) = self.current_module_id() {
            let name: QualifiedName = QualifiedName::new(vec![id.clone()], id.src_ref());
            Some(name.with_prefix(&self.current_namespace()))
        } else {
            None
        }
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
        let mut none: bool = true;
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
            none = false;
            frame.print_stack(f, source_by_hash, idx)?;
        }
        if none {
            writeln!(f, "<empty>")?
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
        for frame in self.0.iter().rev() {
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
                // skip any call frame
                StackFrame::Call {
                    symbol: _,
                    args: _,
                    src_ref: _,
                } => (),
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
                locals.print_locals(f, n * 4)?;
            }
            Ok(())
        }
    }
}

#[test]
#[allow(clippy::unwrap_used)]
fn local_stack() {
    let mut stack = Stack::default();

    let make_int = |id, value| Symbol::new_constant(id, Value::Integer(value));

    let fetch_int = |stack: &Stack, id: &str| -> Option<i64> {
        match stack.fetch(&id.into()) {
            Ok(node) => match &node.borrow().def {
                SymbolDefinition::Constant(_, Value::Integer(value)) => Some(*value),
                _ => todo!("error"),
            },
            _ => None,
        }
    };

    stack.open(StackFrame::Source("test".into(), SymbolMap::default()));
    assert!(stack.current_namespace() == QualifiedName::default());

    assert!(stack.put_local(None, make_int("a".into(), 1)).is_ok());

    println!("{stack}");

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.open(StackFrame::Body(SymbolMap::default()));
    assert!(stack.current_namespace() == QualifiedName::default());

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    assert!(stack.put_local(None, make_int("b".into(), 2)).is_ok());

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").unwrap() == 2);
    assert!(fetch_int(&stack, "c").is_none());

    // test alias
    assert!(stack
        .put_local(Some("x".into()), make_int("x".into(), 3))
        .is_ok());

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").unwrap() == 2);
    assert!(fetch_int(&stack, "x").unwrap() == 3);

    stack.close();
    assert!(stack.current_namespace() == QualifiedName::default());

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.close();
    assert!(stack.current_namespace().is_empty());
}

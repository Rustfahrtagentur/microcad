// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*, resolve::*};

/// A stack with a list of stack frames.
///
/// [`StackFrame`]s can have the following different types:
/// - source file (bottom of stack)
/// - modules ( e.g. `mod my_lib { ... }`)
/// - init calls (e.g. `std::geo2d::Circle(radius = 1m)`)
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
                StackFrame::Source(_, locals)
                | StackFrame::Workbench(_, _, locals)
                | StackFrame::Init(locals)
                | StackFrame::Body(locals)
                | StackFrame::Module(_, locals)
                | StackFrame::Function(locals) => {
                    let op = if locals.insert(id.clone(), symbol).is_some() {
                        "Added"
                    } else {
                        "Set"
                    };
                    if name.is_qualified() {
                        log::debug!("{op} {name} as {id:?} to local stack");
                    } else {
                        log::debug!("{op} {id:?} to local stack");
                    }

                    log::trace!("Local Stack:\n{self}");
                    return Ok(());
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

    /// Get name of current module.
    pub fn current_module_name(&self) -> QualifiedName {
        let mut ids: Vec<Identifier> = Vec::new();
        for frame in self.0.iter().rev() {
            match frame {
                StackFrame::Source(id, _)
                | StackFrame::Module(id, _)
                | StackFrame::Workbench(_, id, _) => ids.push(id.clone()),
                StackFrame::Init(_) | StackFrame::Body(_) | StackFrame::Function(_) => (),
                StackFrame::Call { symbol, .. } => {
                    // RULE: builtins run in the callers context - so skip builtin call frames
                    if !symbol.is_builtin() {
                        ids.extend(symbol.full_base().iter().cloned().rev());
                        break;
                    }
                }
            }
        }
        QualifiedName::no_ref(ids.into_iter().rev().collect())
    }

    /// Get name of current workbench.
    pub fn current_workbench_name(&self) -> Option<QualifiedName> {
        let mut ids: Vec<Identifier> = Vec::new();
        for frame in self.0.iter().rev() {
            match frame {
                StackFrame::Source(id, _) | StackFrame::Module(id, _) => {
                    if ids.is_empty() {
                        // return none if we haven't found a workbench yet
                        return None;
                    } else {
                        // add id to full name of the workbench
                        ids.push(id.clone())
                    }
                }
                StackFrame::Workbench(_, id, _) => ids.push(id.clone()),
                // ignore frames without ids
                StackFrame::Init(_) | StackFrame::Body(_) | StackFrame::Function(_) => (),
                StackFrame::Call { symbol, .. } => {
                    //  finish name
                    ids.extend(symbol.full_base().iter().cloned().rev());
                    // stop here
                    break;
                }
            }
        }
        if ids.is_empty() {
            None
        } else {
            Some(QualifiedName::no_ref(ids.into_iter().rev().collect()))
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
            writeln!(f, crate::invalid!(STACK))?
        }
        Ok(())
    }
}

impl Locals for Stack {
    fn open(&mut self, frame: StackFrame) {
        if let Some(id) = frame.id() {
            log::trace!("Opening {} stack frame '{id}'", frame.kind_str());
        } else {
            log::trace!("Opening {} stack frame", frame.kind_str());
        }
        self.0.push(frame);
    }

    fn close(&mut self) {
        if let Some(frame) = self.0.pop() {
            log::trace!("Closing {} stack frame", frame.kind_str());
        }
    }

    fn set_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()> {
        self.put_local(Some(id.clone()), Symbol::new_constant(id, value))
    }

    fn get_local_value(&self, id: &Identifier) -> EvalResult<Value> {
        match self.fetch(id) {
            Ok(symbol) => match &symbol.borrow().def {
                SymbolDefinition::Constant(_, value) | SymbolDefinition::Argument(_, value) => {
                    Ok(value.clone())
                }
                _ => Err(EvalError::LocalNotFound(id.clone())),
            },
            Err(_) => Err(EvalError::LocalNotFound(id.clone())),
        }
    }

    fn get_model(&self) -> EvalResult<Model> {
        match self
            .0
            .iter()
            .rev()
            .find(|frame| matches!(frame, StackFrame::Workbench(_, _, _)))
        {
            Some(StackFrame::Workbench(model, _, _)) => Ok(model.clone()),
            _ => unreachable!("missing model in workbench"),
        }
    }

    fn fetch(&self, id: &Identifier) -> EvalResult<Symbol> {
        // search from inner scope to root scope to shadow outside locals
        for frame in self.0.iter().rev() {
            match frame {
                StackFrame::Source(_, locals)
                | StackFrame::Body(locals)
                | StackFrame::Workbench(_, _, locals)
                | StackFrame::Init(locals)
                | StackFrame::Function(locals) => {
                    if let Some(local) = locals.get(id) {
                        log::trace!("fetched {id:?} from locals");
                        return Ok(local.clone());
                    }
                }
                // stop stack lookup at calls
                StackFrame::Module(_, _) => {
                    log::trace!("stop at call frame");
                    break;
                }
                // skip any of these
                StackFrame::Call {
                    symbol: _,
                    args: _,
                    src_ref: _,
                } => (),
            }
        }
        Err(EvalError::LocalNotFound(id.clone()))
    }

    /// Get name of current workbench or module (might be empty).
    fn current_name(&self) -> QualifiedName {
        if let Some(name) = self.current_workbench_name() {
            name
        } else {
            self.current_module_name()
        }
    }
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            writeln!(f, crate::invalid!(STACK))
        } else {
            for (n, locals) in self.0.iter().enumerate() {
                locals.print_locals(f, n, 4)?;
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

    let root_name = "test".into();
    let root_id = QualifiedName::from_id(root_name);
    stack.open(StackFrame::Source("test".into(), SymbolMap::default()));
    assert!(stack.current_module_name() == root_id);

    assert!(stack.put_local(None, make_int("a".into(), 1)).is_ok());

    println!("{stack}");

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.open(StackFrame::Body(SymbolMap::default()));
    assert!(stack.current_module_name() == root_id);

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
    assert!(stack.current_module_name() == root_id);

    assert!(fetch_int(&stack, "a").unwrap() == 1);
    assert!(fetch_int(&stack, "b").is_none());
    assert!(fetch_int(&stack, "c").is_none());

    stack.close();
    assert!(stack.current_module_name().is_empty());
}

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

    fn current_workbench_id(&self) -> Option<&Identifier> {
        self.0.iter().rev().find_map(|frame| {
            if let StackFrame::Workbench(_, id, _) = frame {
                Some(id)
            } else {
                None
            }
        })
    }

    /// Get name of current module.
    pub fn current_module_name(&self) -> QualifiedName {
        if self.0.is_empty() {
            QualifiedName::default()
        } else {
            let mut module_name = QualifiedName::default();
            for (n, frame) in self.0.iter().rev().enumerate() {
                match frame {
                    StackFrame::Source(id, ..) | StackFrame::Module(id, ..) => {
                        module_name.insert(0, id.clone());
                    }
                    StackFrame::Call { symbol, .. } => {
                        if n > 0 {
                            // log::trace!("CALL: {}, {}", symbol.full_name(), module_name);
                            module_name =
                                symbol.full_name().remove_last().with_prefix(&module_name);
                            break;
                        }
                    }
                    _ => (),
                }
            }

            // log::trace!("current_module_name: {module_name:?}");
            module_name
        }
    }

    /// Get name of current workbench.
    pub fn current_workbench_name(&self) -> Option<QualifiedName> {
        if let Some(id) = self.current_workbench_id() {
            let name = QualifiedName::new(vec![id.clone()], id.src_ref());
            Some(name.with_prefix(&self.current_module_name()))
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
            writeln!(f, crate::invalid!(STACK))?
        }
        Ok(())
    }

    pub(crate) fn current_symbol(&self) -> Option<Symbol> {
        self.0.iter().rev().find_map(|frame| frame.symbol())
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
        self.put_local(
            Some(id.clone()),
            Symbol::new(
                SymbolDefinition::Constant(Visibility::Private, id, value),
                None,
            ),
        )
    }

    fn get_local_value(&self, id: &Identifier) -> EvalResult<Value> {
        match self.fetch(id) {
            Ok(symbol) => match &symbol.borrow().def {
                SymbolDefinition::Constant(.., value) | SymbolDefinition::Argument(.., value) => {
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
            _ => Err(EvalError::NoModelInWorkbench),
        }
    }

    fn fetch(&self, id: &Identifier) -> EvalResult<Symbol> {
        // search from inner scope to root scope to shadow outside locals
        for (n, frame) in self.0.iter().rev().enumerate() {
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
                } => {
                    if n > 0 {
                        break;
                    }
                }
            }
        }
        Err(EvalError::LocalNotFound(id.clone()))
    }

    /// Get name of current workbench or module (might be empty).
    fn current_name(&self) -> QualifiedName {
        if let Some(id) = self.current_workbench_id() {
            let name = QualifiedName::new(vec![id.clone()], id.src_ref());
            name.with_prefix(&self.current_module_name())
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
                locals.print_locals(f, n, 0)?;
            }
            Ok(())
        }
    }
}

#[test]
#[allow(clippy::unwrap_used)]
fn local_stack() {
    let mut stack = Stack::default();

    let make_int = |id, value| {
        Symbol::new(
            SymbolDefinition::Constant(Visibility::Private, id, Value::Integer(value)),
            None,
        )
    };

    let fetch_int = |stack: &Stack, id: &str| -> Option<i64> {
        match stack.fetch(&id.into()) {
            Ok(node) => match &node.borrow().def {
                SymbolDefinition::Constant(.., Value::Integer(value)) => Some(*value),
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

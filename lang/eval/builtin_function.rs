// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin function evaluation entity

use crate::{eval::*, parse::*, src_ref::SrcRef, sym::*};

/// Type of the functor which receives a call
pub type BuiltinFunctionFn = dyn Fn(&ArgumentMap, &mut Context) -> EvalResult<Option<Value>>;

/// Builtin function
#[derive(Clone)]
pub struct BuiltinFunction {
    /// Name of the builtin function
    pub name: Identifier,
    /// Signature of the builtin function
    pub signature: FunctionSignature,
    /// Functor to evaluate this function
    pub f: &'static BuiltinFunctionFn,
}

impl std::fmt::Debug for BuiltinFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BUILTIN({})", &self.name)
    }
}

impl BuiltinFunction {
    /// Create new builtin function
    pub fn new(
        name: Identifier,
        signature: FunctionSignature,
        f: &'static BuiltinFunctionFn,
    ) -> Self {
        Self { name, signature, f }
    }
}

impl CallTrait for BuiltinFunction {
    type Output = Option<Value>;

    /// Call builtin function with given parameter
    /// # Arguments
    /// - `args`: Function arguments
    /// - `context`: Execution context
    fn call(&self, args: &CallArgumentList, context: &mut Context) -> EvalResult<Option<Value>> {
        let arg_map = args.get_matching_arguments(context, &self.signature.parameters)?;
        let result = (self.f)(&arg_map, context)?;

        match (&result, &self.signature.return_type) {
            (Some(result), Some(return_type)) => {
                if result.ty() != return_type.ty() {
                    use crate::diag::PushDiag;
                    context.error(
                        SrcRef(None),
                        Box::new(EvalError::ReturnTypeMismatch {
                            name: self.name.clone(),
                            expected: return_type.ty(),
                            found: result.ty(),
                        }),
                        Some(context.stack_trace()),
                    )?;
                    Ok(Some(Value::Invalid))
                } else {
                    Ok(Some(result.clone()))
                }
            }
            (Some(result), None) => Ok(Some(result.clone())),
            (None, Some(_)) => Err(EvalError::FunctionCallMissingReturn),
            _ => Ok(None),
        }
    }
}

/// @todo: Check if is possible to rewrite this macro with arbitrary number of arguments
#[macro_export]
macro_rules! builtin_function {
    ($f:ident($name:ident) for $($ty:tt),+) => { BuiltinFunction::new(
        stringify!($f).into(),
        microcad_lang::function_signature!(microcad_lang::parameter_list![microcad_lang::parameter!($name)]),
        &|args, _| {
            match args.get(stringify!($name)) {
                $(Some(Value::$ty($name)) => Ok(Some(Value::$ty(Refer::none($name.$f())))),)*
                Some(Value::List(v)) => {
                    // TODO: Don't use `mut``
                    let mut result = ValueList::new(Vec::new(),SrcRef(None));
                    v.iter().try_for_each(|x| {
                        match x {
                            $(Value::$ty(x) => result.push(Value::$ty(Refer::none(x.$f()))),)*
                            _ => return Err(EvalError::InvalidArgumentType(x.ty())),
                        }
                        Ok(())
                    })?;
                    Ok(Some(Value::List(List::new(result, v.ty(),SrcRef(None)))))
                }
                Some(v) => Err(EvalError::InvalidArgumentType(v.ty())),
                None => Err(EvalError::CannotGetArgument(stringify!($name)))
            }
        })
    };
    ($f:ident($name:ident) $inner:expr) => {
        BuiltinFunction::new(stringify!($f).into(),
        microcad_lang::function_signature!(microcad_lang::parameter_list![microcad_lang::parameter!($name)]),
        &|args, _| {
            let l = |$name| Ok(Some($inner?));
            l(args.get(stringify!($name)).expect("Argument not found").clone())
        })
    };
    ($f:ident($x:ident, $y:ident) $inner:expr) => {
        BuiltinFunction::new(
            stringify!($f).into(),
            microcad_lang::function_signature!(microcad_lang::parameter_list![
                microcad_lang::parameter!($x),
                microcad_lang::parameter!($y)
            ]),
            &|args, _| {
                let l = |$x, $y| Ok(Some($inner?));
                let (x, y) = (
                    args.get(stringify!($x)).unwrap().clone(),
                    args.get(stringify!($y)).unwrap().clone(),
                );
                l(x.clone(), y.clone())
            },
        )
    };
}

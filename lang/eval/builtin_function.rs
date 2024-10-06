// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin function evaluation entity

use crate::{eval::*, parse::*};

/// Type of the functor which receives a call
pub type BuiltinFunctionFn = dyn Fn(&ArgumentMap, &mut Context) -> Result<Option<Value>>;

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
    /// Call builtin function with given parameter
    /// # Arguments
    /// - `args`: Function arguments
    /// - `context`: Execution context
    fn call(&self, args: &CallArgumentList, context: &mut Context) -> Result<Option<Value>> {
        let arg_map = args
            .eval(context)?
            .get_matching_arguments(&self.signature.parameters.eval(context)?)?;
        let result = (self.f)(&arg_map, context)?;

        match (&result, &self.signature.return_type) {
            (Some(result), Some(return_type)) => {
                if result.ty() != return_type.ty() {
                    Err(EvalError::TypeMismatch {
                        expected: return_type.ty(),
                        found: result.ty(),
                    })
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
        match args.get(stringify!($name)).unwrap() {
            $(Value::$ty($name) => Ok(Some(Value::$ty(Refer::none($name.$f())))),)*
            Value::List(v) => {
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
            v => Err(EvalError::InvalidArgumentType(v.ty())),
        }
    })
    };
    ($f:ident($name:ident) $inner:expr) => {
        BuiltinFunction::new(stringify!($f).into(),
        microcad_lang::function_signature!(microcad_lang::parameter_list![microcad_lang::parameter!($name)]),
        &|args, _| {
            let l = |$name| Ok(Some($inner?));
            l(args.get(stringify!($name)).unwrap().clone())
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

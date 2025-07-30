// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin function evaluation entity

use crate::{eval::*, model::*, syntax::*};

enum BuiltinReturn {
    Value(Value),
    Models(Models),
}

/// Builtin function type
pub type BuiltinFn = dyn Fn(
    Option<&ParameterValueList>,
    &ArgumentValueList,
    &mut Context,
) -> EvalResult<BuiltinReturn>;

/// Builtin function struct
#[derive(Clone)]
pub struct Builtin {
    /// Name of the builtin function
    pub id: Identifier,

    /// Optional parameter value list to check the builtin signature.
    pub parameters: Option<ParameterValueList>,

    /// Functor to evaluate this function
    pub f: &'static BuiltinFn,
}

impl std::fmt::Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "__builtin::{}", &self.id)
    }
}

impl Builtin {
    /// Return identifier
    pub fn id(&self) -> Identifier {
        self.id.clone()
    }
}

impl CallTrait for Builtin {
    /// Call builtin function with given parameter
    /// # Arguments
    /// - `args`: Function arguments
    /// - `context`: Execution context
    fn call(&self, args: &ArgumentValueList, context: &mut Context) -> EvalResult<Value> {
        (self.f)(self.parameters.as_ref(), args, context)
    }
}

/// Builtin part definition
pub trait BuiltinWorkbenchDefinition {
    /// Get id of the builtin part
    fn id() -> &'static str;
    /// Create model from argument map
    fn model(args: &Tuple) -> EvalResult<Model>;
    /// Part function
    fn function() -> &'static BuiltinFn {
        &|params, args, _| {
            log::trace!("Built-in workbench call {id:?}({args})", id = Self::id());
            ArgumentMatch::find_multi_match(
                args,
                params.expect("A built-in part must have a parameter list"),
            )?
            .iter()
            .map(|args| {
                Self::model(args).inspect(|model| {
                    model.borrow_mut().origin.arguments = args.clone();
                })
            })
            .collect::<Result<Models, _>>()
        }
    }

    /// Part initialization parameters
    fn parameters() -> ParameterValueList;

    /// Create builtin symbol
    fn symbol() -> Symbol {
        Symbol::new_builtin(
            Identifier::no_ref(Self::id()),
            Some(Self::parameters()),
            Self::function(),
        )
    }
}

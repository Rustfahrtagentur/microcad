// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin function evaluation entity

use crate::{eval::*, model_tree::*, syntax::*};

/// Builtin function type
pub type BuiltinFn = dyn Fn(&CallArgumentValueList, &mut Context) -> EvalResult<Value>;

/// Builtin function struct
#[derive(Clone)]
pub struct Builtin {
    /// Name of the builtin function
    pub id: Identifier,
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
    fn call(&self, args: &CallArgumentValueList, context: &mut Context) -> EvalResult<Value> {
        (self.f)(args, context)
    }
}

/// Builtin part definition
pub trait BuiltinPartDefinition {
    /// Get id of the builtin part
    fn id() -> &'static str;
    /// Create node from argument map
    fn node(args: &ArgumentMap) -> EvalResult<ModelNode>;
    /// Part function
    fn function() -> &'static BuiltinFn {
        &|args, context| {
            let multi_args = args.get_multi_matching_arguments(context, &Self::parameters())?;
            let mut nodes = ModelNodes::default();
            for args in multi_args.combinations() {
                nodes.push(Self::node(&args)?.set_original_arguments(args.clone()));
            }

            Ok(Value::Nodes(nodes))
        }
    }

    /// Part initialization parameters
    fn parameters() -> ParameterList;

    /// Create builtin symbol
    fn symbol() -> Symbol {
        Symbol::new_builtin(Identifier::no_ref(Self::id()), Self::function())
    }
}

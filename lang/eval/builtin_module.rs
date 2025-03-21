// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin module evaluation entity

use crate::{eval::*, objects::*, syntax::*};

/// Builtin module initialization functor
pub type BuiltinModuleFn = dyn Fn(&CallArgumentList, &mut EvalContext) -> EvalResult<ObjectNode>;

/// Builtin module
#[derive(Clone)]
pub struct BuiltinModule {
    /// Name of the module
    pub id: Id,
    /// Module's static builtin function
    pub m: &'static BuiltinModuleFn,
}

impl BuiltinModule {
    pub fn new(id: Id, m: &'static BuiltinModuleFn) -> std::rc::Rc<Self> {
        std::rc::Rc::new(Self { id, m })
    }
}

impl std::fmt::Debug for BuiltinModule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BUILTIN_MOD({})", &self.id)
    }
}

/// Builtin module definition
pub trait BuiltinModuleDefinition {
    /// Get name of the builtin module
    fn name() -> &'static str;
    /// Get parameters of the builtin module (implicit init)
    fn parameters() -> ParameterList;
    /// Create node from argument map
    fn node(args: &CallArgumentList, context: &EvalContext) -> EvalResult<ObjectNode>;
    /// Module function
    fn module() -> &'static BuiltinModuleFn {
        &|args, context| Self::node(args, context)
    }

    /// Generate builtin module
    fn builtin_module() -> BuiltinModule {
        BuiltinModule {
            id: Self::name().into(),
            m: Self::module(),
        }
    }
}

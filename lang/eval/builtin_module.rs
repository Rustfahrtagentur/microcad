// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin module entity

use crate::{eval::*, objects::*, rc::*, syntax::*};

/// Builtin module initialization functor
pub type BuiltinModuleFn = dyn Fn(&CallArgumentList, &mut EvalContext) -> EvalResult<ObjectNode>;

/// Builtin module
#[derive(Clone)]
pub struct BuiltinModule {
    /// Name of the module
    pub id: Identifier,
    /// Module's static builtin function
    pub m: &'static BuiltinModuleFn,
}

impl BuiltinModule {
    /// Create new builtin module
    pub fn new(id: Identifier, m: &'static BuiltinModuleFn) -> Rc<Self> {
        Rc::new(Self { id, m })
    }
}

impl std::fmt::Debug for BuiltinModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "__builtin::{}", &self.id)
    }
}

/// Builtin module definition
pub trait BuiltinModuleDefinition {
    /// Get name of the builtin module
    fn name() -> &'static str;
    /// Create node from argument map
    fn node(args: &CallArgumentList, context: &EvalContext) -> EvalResult<ObjectNode>;
    /// Module function
    fn module() -> &'static BuiltinModuleFn {
        &|args, context| Self::node(args, context)
    }

    /// Generate builtin module
    fn builtin_module() -> BuiltinModule {
        BuiltinModule {
            id: Identifier(Refer::none(Self::name().into())),
            m: Self::module(),
        }
    }
}

// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin module evaluation entity

use crate::{eval::*, objects::*, syntax::*};

/// Builtin module initialization functor
pub type BuiltinModuleFn = dyn Fn(&ArgumentMap, &mut EvalContext) -> EvalResult<ObjectNode>;

/// Builtin module
#[derive(Clone)]
pub struct BuiltinModule {
    /// Name of the module
    pub name: Identifier,
    /// Module parameters
    pub parameters: ParameterList,
    /// Module's implicit initialization
    pub f: &'static BuiltinModuleFn,
}

impl std::fmt::Debug for BuiltinModule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BUILTIN_MOD({})", &self.name)
    }
}

/// Builtin module definition
pub trait BuiltinModuleDefinition {
    /// Get name of the builtin module
    fn name() -> &'static str;
    /// Get parameters of the builtin module (implicit init)
    fn parameters() -> ParameterList;
    /// Create node from argument map
    fn node(args: &ArgumentMap) -> EvalResult<ObjectNode>;
    /// Implicit initialization functor
    fn function() -> &'static BuiltinModuleFn {
        &|args, _ctx| Self::node(args)
    }
    /// Generate builtin module
    fn builtin_module() -> BuiltinModule {
        BuiltinModule {
            name: Self::name().into(),
            parameters: Self::parameters(),
            f: Self::function(),
        }
    }
}

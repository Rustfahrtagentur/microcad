// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of parsed content

mod builtin_function;
mod builtin_module;
mod call;
mod errors;
mod eval_context;
mod parameter;
mod ty;
mod value;

pub use builtin_function::*;
pub use builtin_module::*;
pub use call::*;
pub use errors::*;
pub use eval_context::*;
pub use parameter::*;
pub use ty::*;
pub use value::*;

/// Evaluation trait
pub trait Eval {
    /// Implementor's ok result type
    type Output;

    /// Evaluate the type into an expression
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Self::Output>;
}

pub use crate::sym::Id;

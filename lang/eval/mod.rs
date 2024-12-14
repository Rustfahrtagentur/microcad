// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation of parsed content

#![warn(missing_docs, clippy::unwrap_used)]

mod builtin_function;
mod builtin_module;
mod call;
mod context;
mod errors;
mod parameter;
mod symbols;
mod ty;
mod value;

pub use builtin_function::*;
pub use builtin_module::*;
pub use call::*;
pub use context::*;
pub use errors::*;
pub use parameter::*;
pub use symbols::*;
pub use ty::*;
pub use value::*;

/// Evaluation trait
pub trait Eval {
    /// Implementor's ok result type
    type Output;

    /// Evaluate the type into an expression
    fn eval(&self, context: &mut Context) -> EvalResult<Self::Output>;
}

pub use microcad_core::Id;

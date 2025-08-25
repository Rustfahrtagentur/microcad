// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument value evaluation entity

use crate::eval::*;

/// Trait for calls with argument list.
pub trait CallTrait<ReturnType = Value> {
    /// Evaluate call into value (if possible).
    fn call(&self, args: &ArgumentValueList, context: &mut Context) -> EvalResult<ReturnType>;
}

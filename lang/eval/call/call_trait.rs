// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call argument value evaluation entity

use crate::eval::*;

/// Trait for calls with argument list.
pub trait CallTrait<T = Value> {
    /// Evaluate call into value (if possible).
    fn call(&self, args: &CallArgumentValueList, context: &mut Context) -> EvalResult<T>;
}

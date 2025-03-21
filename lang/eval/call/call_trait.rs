// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Call argument value evaluation entity

use crate::{eval::*, syntax::*};

/// trait for calls of modules or functions with argument list
pub trait CallTrait {
    /// Evaluate call into value (if possible)
    fn call(&self, args: &CallArgumentList, context: &mut EvalContext) -> EvalResult<Value>;
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad value related evaluation entities

use crate::{eval::*, syntax::*};

impl Argument {
    /// Evaluate `Argument` and return `ArgumentValue`
    pub fn eval_value(&self, context: &mut EvalContext) -> EvalResult<ArgumentValue> {
        Ok(ArgumentValue::new(
            self.value.eval(context)?,
            self.value.single_identifier().cloned(),
            self.src_ref.clone(),
        ))
    }
}

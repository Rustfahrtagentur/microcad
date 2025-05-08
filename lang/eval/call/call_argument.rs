// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad value related evaluation entities

use crate::{eval::*, syntax::*};

impl CallArgument {
    /// Evaluate `CallArgument` and return `CallArgumentValue`
    pub fn eval_value(&self, context: &mut Context) -> EvalResult<CallArgumentValue> {
        Ok(CallArgumentValue::new(
            self.id.clone(),
            self.value.eval(context)?,
            self.src_ref.clone(),
        ))
    }
}

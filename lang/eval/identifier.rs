// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval for QualifiedName {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        context.fetch_value(self)
    }
}

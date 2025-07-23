// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::eval::*;

impl Eval<Value> for ReturnStatement {
    fn eval(&self, _context: &mut Context) -> EvalResult<Value> {
        todo!("implement return eval")
    }
}

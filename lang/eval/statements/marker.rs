// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

impl Eval<Models> for Marker {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        if self.is_input_marker() {
            Ok(context.get_input().clone())
        } else {
            Ok(Models::default())
        }
    }
}

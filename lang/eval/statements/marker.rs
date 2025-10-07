// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

impl Eval<Option<Model>> for Marker {
    fn eval(&self, _: &mut EvalContext) -> EvalResult<Option<Model>> {
        if self.is_input_placeholder() {
            Ok(Some(
                ModelBuilder::new(Element::InputPlaceholder, self.src_ref()).build(),
            ))
        } else {
            Ok(None)
        }
    }
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

impl Eval<Option<Model>> for Marker {
    fn eval(&self, _: &mut Context) -> EvalResult<Option<Model>> {
        if self.is_children_marker() {
            Ok(Some(
                ModelBuilder::new(Element::ChildrenMarker, self.src_ref()).build(),
            ))
        } else {
            Ok(None)
        }
    }
}

impl Eval<Models> for Marker {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        let model: Option<Model> = self.eval(context)?;
        Ok(model.into())
    }
}

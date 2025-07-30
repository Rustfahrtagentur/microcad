// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*};

impl Eval<Models> for ModelAssignmentStatement {
    fn eval(&self, context: &mut Context) -> EvalResult<Models> {
        log::debug!("Evaluating model assignment statement:\n{self}");
        // TODO: context.grant(self)?;

        let assignment = &self.assignment;

        let mut models = assignment.expression.eval(context)?;

        let attributes = self.attribute_list.eval(context)?;
        models.iter_mut().for_each(|model| {
            model.borrow_mut().attributes = attributes.clone();
        });

        if let Err(err) = context.set_local_models(assignment.id.clone(), models) {
            context.error(self, err)?;
        }

        Ok([].into_iter().collect())
    }
}

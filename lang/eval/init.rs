// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, model::*, syntax::*};

impl InitDefinition {
    /// Evaluate a call to the init definition
    pub fn eval(
        &self,
        plan: &ParameterList,
        args: Tuple,
        context: &mut Context,
    ) -> EvalResult<ObjectProperties> {
        context.grant(self)?;
        let model = context.get_model()?;
        context.scope(StackFrame::Init(args.into()), |context| {
            // avoid body stack frame
            let result: Value = self.body.statements.eval(context)?;
            // paranoia check
            assert!(result == Value::None);

            let (found, not_found): (Vec<_>, Vec<_>) = plan
                .iter()
                .map(|param| (&param.id, context.get_local_value(&param.id)))
                .partition(|(_, v)| v.is_ok());

            if not_found.is_empty() {
                let props: ObjectProperties = found
                    .into_iter()
                    .map(|(id, value)| ((*id).clone(), value.expect("ok")))
                    .collect();

                model.borrow_mut().set_properties(props.clone());
                Ok(props)
            } else {
                Err(EvalError::BuildingPlanIncomplete(
                    not_found.iter().map(|(id, _)| (*id).clone()).collect(),
                ))
            }
        })
    }
}

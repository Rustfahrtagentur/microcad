// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element evaluation

use crate::{eval::*, model::*, syntax::*};

impl WorkbenchDefinition {
    /// Try to evaluate a single call into a [`Model`].
    ///
    /// - `arguments`: Single argument tuple (will not be multiplied).
    /// - `init`: Initializer to call with given `arguments`.
    /// - `context`: Current evaluation context.
    fn eval_to_model<'a>(
        &'a self,
        arguments: Tuple,
        init: Option<&'a InitDefinition>,
        context: &mut Context,
    ) -> EvalResult<Model> {
        log::debug!(
            "Evaluating model of `{id:?}` {kind}",
            id = self.id,
            kind = self.kind
        );

        let (properties,non_properties) :(Vec<_>,Vec<_>) =                 // copy all arguments which are part of the building plan into properties
                arguments
                    .named_iter().map(|(id, value)|(id.clone(),value.clone()))
                    .partition(|(id, _)| {
                        self.plan.contains_key(id)
                    });

        log::trace!("Properties:\n{:?}", properties);
        log::trace!("Non-Properties:\n{:?}", non_properties);

        // Create model
        let model = ModelBuilder::new_workpiece(*self.kind)
            .origin(Origin::new(arguments.clone()))
            .attributes(self.attribute_list.eval(context)?)
            .properties(properties.into_iter().collect())
            .build();

        context.scope(
            StackFrame::Workbench(
                model,
                self.id.clone(),
                non_properties.clone().into_iter().collect(),
            ),
            |context| {
                let model = context.get_model()?;

                // run init code
                if let Some(init) = init {
                    log::trace!(
                        "Initializing`{id:?}` {kind}",
                        id = self.id,
                        kind = self.kind
                    );
                    if let Err(err) =
                        init.eval(&self.plan, non_properties.into_iter().collect(), context)
                    {
                        context.error(self.src_ref_head(), err)?;
                    }
                }

                // At this point, all properties must have a value
                log::trace!("Run body`{id:?}` {kind}", id = self.id, kind = self.kind);
                model.append_children(self.body.statements.eval(context)?);

                // We have to deduce the output type of this model, otherwise the model is incomplete.
                {
                    let output_type = model.deduce_output_type();
                    let model_ = model.borrow();
                    match &model_.element {
                        Element::Workpiece(workpiece) => {
                            let result = workpiece.check_output_type(output_type);
                            match result {
                                Ok(()) => {}
                                Err(EvalError::WorkbenchNoOutput(..)) => {
                                    context
                                        .warning(self.src_ref_head(), result.expect_err("Error"))?;
                                }
                                result => {
                                    context
                                        .error(self.src_ref_head(), result.expect_err("Error"))?;
                                }
                            }
                        }
                        _ => panic!("A workbench must produce a workpiece."),
                    }
                }

                Ok(model)
            },
        )
    }
}

impl CallTrait<Models> for WorkbenchDefinition {
    /// Evaluate the call of a workbench with given arguments.
    ///
    /// - `args`: Arguments which will be matched with the building plan and the initializers using parameter multiplicity.
    /// - `context`: Current evaluation context.
    ///
    /// Return evaluated nodes (multiple nodes might be created by parameter multiplicity).
    fn call(&self, args: &ArgumentValueList, context: &mut Context) -> EvalResult<Models> {
        log::debug!(
            "Workbench {call} {kind} {id:?}({args})",
            call = crate::mark!(CALL),
            id = self.id,
            kind = self.kind
        );

        // prepare models
        let mut models = Models::default();
        // prepare building plan
        let plan = self.plan.eval(context)?;

        // try to match arguments with the building plan
        match ArgumentMatch::find_multi_match(args, &plan) {
            Ok(matches) => {
                log::debug!(
                    "Building plan matches: {}",
                    matches
                        .iter()
                        .map(|m| m.to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                // evaluate models for all multiplicity matches
                for args in matches {
                    models.push(self.eval_to_model(args, None, context)?);
                }
            }
            _ => {
                log::trace!("Building plan did not match, finding initializer");

                // at the end: check if initialization was successful
                let mut initialized = false;

                // find an initializer that matches the arguments
                for init in self.inits() {
                    if let Ok(matches) =
                        ArgumentMatch::find_multi_match(args, &init.parameters.eval(context)?)
                    {
                        log::debug!(
                            "Initializer matches: {}",
                            matches
                                .iter()
                                .map(|m| m.to_string())
                                .collect::<Vec<_>>()
                                .join("\n")
                        );
                        // evaluate models for all multiplicity matches
                        for args in matches {
                            models.push(self.eval_to_model(args, Some(init), context)?);
                        }
                        initialized = true;
                        break;
                    }
                }
                if !initialized {
                    context.error(args, EvalError::NoInitializationFound(self.id.clone()))?;
                }
            }
        }

        Ok(models)
    }
}

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
            "Evaluating model of `{id}` {kind}",
            id = self.id,
            kind = self.kind
        );

        // Create model
        let model_builder: ModelBuilder = self.kind.into();
        let model = model_builder
            .origin(Origin {
                arguments: arguments.clone(),
                // TODO: where to get the rest?
                ..Default::default()
            })
            .attributes(self.attribute_list.eval(context)?)
            .properties(
                // copy all arguments which are part of the building plan to properties
                arguments
                    .named_iter()
                    .filter_map(|(id, arg)| {
                        if self.plan.contains_key(id) {
                            Some((id.clone(), arg.clone()))
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
            .build();

        context.scope(
            StackFrame::Workbench(model, self.id.clone(), arguments.clone().into()),
            |context| {
                let model = context.get_model()?;

                // run init code
                if let Some(init) = init {
                    log::trace!("Initializing`{id}` {kind}", id = self.id, kind = self.kind);
                    if let Err(err) = match init.eval(&self.plan, arguments.clone(), context) {
                        Ok(props) => props.iter().try_for_each(|(id, value)| {
                            context.set_local_value(id.clone(), value.clone())
                        }),
                        Err(err) => context.error(init, err),
                    } {
                        context.error(self, err)?;
                    }
                }

                // At this point, all properties must have a value
                log::trace!("Run body`{id}` {kind}", id = self.id, kind = self.kind);
                model.append_children(self.body.statements.eval(context)?);

                // We have to deduce the output type of this model, otherwise the model is incomplete.
                model.deduce_output_type();

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
            "Workbench call {kind} {id}({args})",
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

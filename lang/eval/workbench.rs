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
        call_src_ref: SrcRef,
        creator: Creator,
        init: Option<&'a InitDefinition>,
        context: &mut Context,
    ) -> EvalResult<Model> {
        log::debug!(
            "Evaluating model of `{id:?}` {kind}",
            id = self.id,
            kind = self.kind
        );

        let arguments = creator.arguments.clone();

        // Create model
        let model = ModelBuilder::new(
            Element::Workpiece(Workpiece {
                kind: self.kind,

                // copy all arguments which are part of the building plan to properties
                properties: creator
                    .arguments
                    .named_iter()
                    .filter_map(|(id, arg)| {
                        if self.plan.contains_key(id) {
                            Some((id.clone(), arg.clone()))
                        } else {
                            None
                        }
                    })
                    .collect(),
                creator,
            }),
            call_src_ref,
        )
        .attributes(self.attribute_list.eval(context)?)
        .build();

        context.scope(
            StackFrame::Workbench(model, self.id.clone(), arguments.clone().into()),
            |context| {
                let model = context.get_model()?;

                // run init code
                if let Some(init) = init {
                    log::trace!(
                        "Initializing`{id:?}` {kind}",
                        id = self.id,
                        kind = self.kind
                    );
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
                log::trace!("Run body`{id:?}` {kind}", id = self.id, kind = self.kind);
                model.append_children(self.body.statements.eval(context)?);

                // We have to deduce the output type of this model, otherwise the model is incomplete.
                {
                    let model_ = model.borrow();
                    match &*model_.element {
                        Element::Workpiece(workpiece) => {
                            let output_type = model.deduce_output_type();

                            let result = workpiece.check_output_type(output_type);
                            match result {
                                Ok(()) => {}
                                Err(EvalError::WorkbenchNoOutput(_, _)) => {
                                    context.warning(self, result.expect_err("Error"))?;
                                }
                                result => {
                                    context.error(self, result.expect_err("Error"))?;
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

impl WorkbenchDefinition {
    /// Evaluate the call of a workbench with given arguments.
    ///
    /// - `args`: Arguments which will be matched with the building plan and the initializers using parameter multiplicity.
    /// - `context`: Current evaluation context.
    ///
    /// Return evaluated nodes (multiple nodes might be created by parameter multiplicity).
    pub fn call(
        &self,
        call_src_ref: SrcRef,
        symbol: Symbol,
        arguments: &ArgumentValueList,
        context: &mut Context,
    ) -> EvalResult<Model> {
        log::debug!(
            "Workbench {call} {kind} {id:?}({arguments})",
            call = crate::mark!(CALL),
            id = self.id,
            kind = self.kind
        );

        // prepare models
        let mut models = Models::default();
        // prepare building plan
        let plan = self.plan.eval(context)?;

        // try to match arguments with the building plan
        match ArgumentMatch::find_multi_match(arguments, &plan) {
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
                for arguments in matches {
                    models.push(self.eval_to_model(
                        call_src_ref.clone(),
                        Creator::new(symbol.clone(), arguments),
                        None,
                        context,
                    )?);
                }
            }
            _ => {
                log::trace!("Building plan did not match, finding initializer");

                // at the end: check if initialization was successful
                let mut initialized = false;

                // find an initializer that matches the arguments
                for init in self.inits() {
                    if let Ok(matches) =
                        ArgumentMatch::find_multi_match(arguments, &init.parameters.eval(context)?)
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
                        for arguments in matches {
                            models.push(self.eval_to_model(
                                call_src_ref.clone(),
                                Creator::new(symbol.clone(), arguments),
                                Some(init),
                                context,
                            )?);
                        }
                        initialized = true;
                        break;
                    }
                }
                if !initialized {
                    context.error(arguments, EvalError::NoInitializationFound(self.id.clone()))?;
                }
            }
        }

        Ok(models.to_multiplicity(self.src_ref.clone()))
    }
}

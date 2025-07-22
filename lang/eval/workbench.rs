// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Workbench definition syntax element evaluation

use crate::{eval::*, model::*, syntax::*};

impl WorkbenchDefinition {
    /// Try to evaluate a single call into a [`Model`].
    fn eval_to_model<'a>(
        &'a self,
        arguments: &Tuple,
        init: Option<&'a InitDefinition>,
        context: &mut Context,
    ) -> EvalResult<Model> {
        log::debug!(
            "Evaluating to model `{id}` {kind}",
            id = self.id,
            kind = self.kind
        );
        let model_builder = match self.kind {
            WorkbenchKind::Part => ModelBuilder::new_3d_object(),
            WorkbenchKind::Sketch => ModelBuilder::new_2d_object(),
            WorkbenchKind::Operation => ModelBuilder::new_object_body(),
        }
        .properties(ObjectProperties::from_parameters_and_arguments(
            &self.plan.eval(context)?,
            arguments,
        ));

        context.scope(
            StackFrame::Workbench(model_builder.into(), self.id.clone(), arguments.into()),
            |context| {
                // Create the object model from initializer if present
                if let Some(init) = init {
                    log::trace!("Initializing`{id}` {kind}", id = self.id, kind = self.kind);
                    init.eval(arguments, context)?;
                }

                let model_builder = context.get_model_builder()?;
                {
                    let mut model_builder = model_builder.borrow_mut();
                    model_builder.properties.eval(context)?;

                    // At this point, all properties must have a value
                    log::trace!("Run body`{id}` {kind}", id = self.id, kind = self.kind);
                    for statement in self.body.statements.iter() {
                        match statement {
                            Statement::Assignment(assignment) => {
                                assignment.eval(context)?;
                            }
                            Statement::Expression(expression) => {
                                model_builder.add_children2(expression.eval(context)?)?;
                            }
                            Statement::Init(_) => (),
                            _ => todo!("Evaluate statement: {statement}"),
                        }
                    }
                }
                let model = model_builder.take().build();
                {
                    let mut model_ = model.borrow_mut();
                    model_.origin.arguments = arguments.clone();
                    model_.attributes = self.attribute_list.eval(context)?;
                }
                Ok(model)
            },
        )
    }
}

impl CallTrait<Models> for WorkbenchDefinition {
    /// Evaluate the call of a part initialization
    ///
    /// The evaluation considers multiplicity, which means that multiple models maybe created.
    ///
    /// Example:
    /// Consider the `part a(b: Scalar) { }`.
    /// Calling the part `a([1.0, 2.0])` results in two models with `b = 1.0` and `b = 2.0`, respectively.
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

        let mut initialized = args.is_empty();

        if let Ok(multi_args) = ArgumentMatch::find_multi_match(args, &plan) {
            for args in multi_args {
                for (id, var) in args.named_iter() {
                    context.set_local_value(id.clone(), var.clone())?;
                }
                models.push(self.eval_to_model(&args, None, context)?);
            }
            initialized = true;
        } else {
            // put all default parameters in the building plan into local variables
            plan.iter().try_for_each(|(id, arg)| {
                if let Some(def) = &arg.default_value {
                    context.set_local_value(id.clone(), def.clone())?;
                }
                EvalResult::Ok(())
            })?;

            for init in self.inits() {
                if let Ok(multi_args) =
                    ArgumentMatch::find_multi_match(args, &init.parameters.eval(context)?)
                {
                    for args in multi_args {
                        models.push(self.eval_to_model(&args, Some(init), context)?);
                    }
                    initialized = true;
                    break;
                }
            }
        }
        if !initialized {
            context.error(args, EvalError::NoInitializationFound(self.id.clone()))?;
        }

        Ok(models)
    }
}

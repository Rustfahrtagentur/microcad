// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::str::FromStr;

use crate::{
    Id,
    builtin::ExporterAccess,
    eval::{self, *},
    model::{Attributes, ExportCommand},
    parameter,
    syntax::{self, *},
};

use microcad_core::{Color, RenderResolution, Size2D};
use thiserror::Error;

/// Error type for attributes.
#[derive(Debug, Error)]
pub enum AttributeError {
    /// Unknown attribute.
    #[error("Attribute not supported: {0}")]
    NotSupported(Identifier),

    /// Attribute cannot be assigned to an expression.
    #[error("Cannot assign attribute to expression `{0}`")]
    CannotAssignToExpression(Box<Expression>),

    /// Warning when an attribute has already been set.
    #[error("The attribute is already set: {0} = {1}")]
    AttributeAlreadySet(Identifier, Value),

    /// The attribute was not found.
    #[error("Not found: {0}")]
    NotFound(Identifier),

    /// Invalid command.
    #[error("Invalid command list for attribute `{0}`")]
    InvalidCommand(Identifier),
}

impl Eval<ArgumentValueList> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<ArgumentValueList> {
        let mut arguments = Vec::new();

        for command in &self.commands {
            match command {
                AttributeCommand::Call(_, _) => todo!(),
                AttributeCommand::Expression(expression) => arguments.push((
                    Identifier::default(),
                    ArgumentValue::new(expression.eval(context)?, expression.src_ref()),
                )),
            }
        }

        Ok(ArgumentValueList::new(arguments, self.src_ref()))
    }
}

impl Eval<Option<ExportCommand>> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<ExportCommand>> {
        match ArgumentMatch::find_match(
            &self.eval(context)?,
            &[
                parameter!(filename: String),
                parameter!(resolution: Length = 0.1 /*mm*/),
                parameter!(id: String = String::new()),
                (
                    Identifier::no_ref("size"),
                    eval::ParameterValue {
                        specified_type: Some(Type::Tuple(Box::new(TupleType::new_size2d()))),
                        default_value: Some(Value::Tuple(Box::new(Size2D::A4.into()))),
                        src_ref: SrcRef(None),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        ) {
            Ok(arguments) => {
                let filename: std::path::PathBuf = arguments.get::<String>("filename").into();
                let id: Id = arguments.get::<String>("id").into();
                let id: Option<Id> = if id.is_empty() { None } else { Some(id) };
                let resolution =
                    RenderResolution::new(arguments.get::<&Value>("resolution").try_scalar()?);
                let size = arguments.get::<Size2D>("size");

                match context.find_exporter(&filename, &id) {
                    Ok(exporter) => {
                        return Ok(Some(ExportCommand {
                            filename,
                            exporter,
                            resolution,
                            size,
                            layers: vec![], // TODO get layers
                        }));
                    }
                    Err(err) => context.warning(self, err)?,
                }
            }
            Err(err) => context.warning(self, err)?,
        }

        Ok(None)
    }
}

impl Eval<Option<Color>> for syntax::AttributeCommand {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<Color>> {
        match self {
            AttributeCommand::Call(_, _) => todo!(),
            // Get color from a tuple or string.
            AttributeCommand::Expression(expression) => {
                let value: Value = expression.eval(context)?;
                match value {
                    Value::String(s) => match Color::from_str(&s) {
                        Ok(color) => Ok(Some(color)),
                        Err(err) => {
                            context.warning(self, err)?;
                            Ok(None)
                        }
                    },
                    Value::Tuple(tuple) => match Color::try_from(tuple.as_ref()) {
                        Ok(color) => Ok(Some(color)),
                        Err(err) => {
                            context.warning(self, err)?;
                            Ok(None)
                        }
                    },
                    _ => {
                        context.warning(
                            self,
                            AttributeError::InvalidCommand(Identifier::no_ref("color")),
                        )?;
                        Ok(None)
                    }
                }
            }
        }
    }
}

impl Eval<Option<Color>> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<Color>> {
        assert_eq!(self.id.id().as_str(), "color");
        match self.single_command() {
            Some(command) => Ok(command.eval(context)?),
            None => {
                context.warning(self, AttributeError::InvalidCommand(self.id.clone()))?;
                Ok(None)
            }
        }
    }
}

impl Eval<Option<crate::model::Attribute>> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<crate::model::Attribute>> {
        let id = self.id.id().as_str();
        use crate::model::Attribute as Attr;
        Ok(match id {
            "color" => {
                let color: Option<Color> = self.eval(context)?;
                color.map(Attr::Color)
            }
            "export" => {
                let export: Option<ExportCommand> = self.eval(context)?;
                export.map(Attr::Export)
            }
            _ => todo!(), /*             "resolution" => Attr::Resolution(self.eval(context)?),
                          "theme" => Attr::Theme(self.eval(context)?),
                          "size" => Attr::Size(self.eval(context)?),
                          "export" => Attr::Export(self.eval(context)?),
                          "measure" => Attr::Measure(self.eval(context)?),
                          _ => Attr::Custom(self.id.clone(), self.eval(context)?), */
        })
    }
}

impl Eval<crate::model::Attributes> for AttributeList {
    fn eval(&self, context: &mut Context) -> EvalResult<crate::model::Attributes> {
        let mut attributes = Vec::new();

        for attribute in self.iter() {
            if let Some(attribute) = attribute.eval(context)? {
                attributes.push(attribute);
            }
        }

        Ok(Attributes(attributes))
    }
}

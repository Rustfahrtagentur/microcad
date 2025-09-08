// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::str::FromStr;

use crate::{
    Id,
    builtin::ExporterAccess,
    eval::{self, *},
    model::{Attributes, CustomCommand, ExportCommand, MeasureCommand, ResolutionAttribute},
    parameter,
    syntax::{self, *},
};

use microcad_core::{Color, RenderResolution, Size2, theme::Theme};
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

impl Eval<Option<ExportCommand>> for syntax::AttributeCommand {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<ExportCommand>> {
        match self {
            AttributeCommand::Call(_, Some(argument_list)) => {
                match ArgumentMatch::find_match(
                    &argument_list.eval(context)?,
                    &[
                        parameter!(filename: String),
                        parameter!(resolution: Length = 0.1 /*mm*/),
                        (
                            Identifier::no_ref("size"),
                            eval::ParameterValue {
                                specified_type: Some(Type::Tuple(Box::new(TupleType::new_size2()))),
                                default_value: Some(Value::Tuple(Box::new(Size2::A4.into()))),
                                src_ref: SrcRef(None),
                            },
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ) {
                    Ok(arguments) => {
                        let filename: std::path::PathBuf =
                            arguments.get::<String>("filename").into();
                        let id: Option<Id> = if let Ok(id) = arguments.by_str::<String>("id") {
                            Some(id.into())
                        } else {
                            None
                        };
                        let resolution = RenderResolution::new(
                            arguments.get::<&Value>("resolution").try_scalar()?,
                        );

                        match context.find_exporter(&filename, &id) {
                            Ok(exporter) => Ok(Some(ExportCommand {
                                filename,
                                exporter,
                                resolution,
                            })),
                            Err(err) => {
                                context.warning(self, err)?;
                                Ok(None)
                            }
                        }
                    }
                    Err(err) => {
                        context.warning(self, err)?;
                        Ok(None)
                    }
                }
            }
            AttributeCommand::Expression(expression) => {
                let value: Value = expression.eval(context)?;
                match value {
                    Value::String(filename) => {
                        let filename = std::path::PathBuf::from(filename);
                        match context.find_exporter(&filename, &None) {
                            Ok(exporter) => Ok(Some(ExportCommand {
                                filename,
                                resolution: RenderResolution::default(),
                                exporter,
                            })),
                            Err(err) => {
                                context.warning(self, err)?;
                                Ok(None)
                            }
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            _ => Ok(None),
        }
    }
}

impl Eval<Vec<ExportCommand>> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Vec<ExportCommand>> {
        assert_eq!(self.id.id().as_str(), "export");

        self.commands
            .iter()
            .try_fold(Vec::new(), |mut commands, attribute| {
                if let Some(export_command) = attribute.eval(context)? {
                    commands.push(export_command)
                }
                Ok(commands)
            })
    }
}

impl Eval<Vec<MeasureCommand>> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Vec<MeasureCommand>> {
        let mut commands = Vec::new();

        for command in &self.commands {
            match command {
                AttributeCommand::Call(Some(id), _) => match id.id().as_str() {
                    "width" => commands.push(MeasureCommand::Width),
                    "height" => commands.push(MeasureCommand::Height),
                    "size" => commands.push(MeasureCommand::Size),
                    _ => context.warning(self, AttributeError::InvalidCommand(id.clone()))?,
                },
                _ => unimplemented!(),
            }
        }

        Ok(commands)
    }
}

impl Eval<Vec<CustomCommand>> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Vec<CustomCommand>> {
        match context.exporters().exporter_by_id(self.id.id()) {
            Ok(exporter) => {
                let mut commands = Vec::new();
                for command in &self.commands {
                    match command {
                        AttributeCommand::Call(None, Some(argument_list)) => {
                            match ArgumentMatch::find_match(
                                &argument_list.eval(context)?,
                                &exporter.model_parameters(),
                            ) {
                                Ok(tuple) => commands.push(CustomCommand {
                                    id: self.id.clone(),
                                    arguments: Box::new(tuple),
                                }),
                                Err(err) => {
                                    context.warning(self, err)?;
                                }
                            }
                        }
                        _ => unimplemented!(),
                    }
                }

                Ok(commands)
            }
            Err(err) => {
                context.warning(self, err)?;
                Ok(Vec::default())
            }
        }
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
                    // Color from string: color = "red"
                    Value::String(s) => match Color::from_str(&s) {
                        Ok(color) => Ok(Some(color)),
                        Err(err) => {
                            context.warning(self, err)?;
                            Ok(None)
                        }
                    },
                    // Color from tuple: color = (r = 1.0, g = 1.0, b = 1.0, a = 1.0)
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

impl Eval<Option<ResolutionAttribute>> for syntax::AttributeCommand {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<ResolutionAttribute>> {
        match self {
            AttributeCommand::Expression(expression) => {
                let value: Value = expression.eval(context)?;
                match value {
                    Value::Quantity(qty) => match qty.quantity_type {
                        QuantityType::Scalar => Ok(Some(ResolutionAttribute::Relative(qty.value))),
                        QuantityType::Length => Ok(Some(ResolutionAttribute::Linear(qty.value))),
                        _ => unimplemented!(),
                    },
                    _ => todo!("Error handling"),
                }
            }
            AttributeCommand::Call(_, _) => {
                context.warning(
                    self,
                    AttributeError::InvalidCommand(Identifier::no_ref("resolution")),
                )?;
                Ok(None)
            }
        }
    }
}

impl Eval<Option<std::rc::Rc<Theme>>> for syntax::AttributeCommand {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<std::rc::Rc<Theme>>> {
        match self {
            AttributeCommand::Expression(_) => todo!(),
            AttributeCommand::Call(_, _) => {
                context.warning(
                    self,
                    AttributeError::InvalidCommand(Identifier::no_ref("resolution")),
                )?;
                Ok(None)
            }
        }
    }
}

impl Eval<Option<Size2>> for syntax::AttributeCommand {
    fn eval(&self, _: &mut Context) -> EvalResult<Option<Size2>> {
        todo!("Get Size2, e.g. `size = (width = 10mm, height = 10mm) from AttributeCommand")
    }
}

macro_rules! eval_to_attribute {
    ($id:ident: $ty:ty) => {
        impl Eval<Option<$ty>> for syntax::Attribute {
            fn eval(&self, context: &mut Context) -> EvalResult<Option<$ty>> {
                assert_eq!(self.id.id().as_str(), stringify!($id));
                match self.single_command() {
                    Some(command) => Ok(command.eval(context)?),
                    None => {
                        context.warning(self, AttributeError::InvalidCommand(self.id.clone()))?;
                        Ok(None)
                    }
                }
            }
        }
    };
}

eval_to_attribute!(color: Color);
eval_to_attribute!(resolution: ResolutionAttribute);
eval_to_attribute!(theme: std::rc::Rc<Theme>);
eval_to_attribute!(size: Size2);

impl Eval<Vec<crate::model::Attribute>> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Vec<crate::model::Attribute>> {
        let id = self.id.id().as_str();
        use crate::model::Attribute as Attr;
        Ok(match id {
            "color" => match self.eval(context)? {
                Some(color) => vec![Attr::Color(color)],
                None => Default::default(),
            },
            "resolution" => match self.eval(context)? {
                Some(resolution) => vec![Attr::Resolution(resolution)],
                None => Default::default(),
            },
            "theme" => match self.eval(context)? {
                Some(theme) => vec![Attr::Theme(theme)],
                None => Default::default(),
            },
            "size" => match self.eval(context)? {
                Some(size) => vec![Attr::Size(size)],
                None => Default::default(),
            },
            "export" => {
                let exports: Vec<ExportCommand> = self.eval(context)?;
                exports.iter().cloned().map(Attr::Export).collect()
            }
            "measure" => {
                let measures: Vec<MeasureCommand> = self.eval(context)?;
                measures.iter().cloned().map(Attr::Measure).collect()
            }
            _ => {
                let commands: Vec<CustomCommand> = self.eval(context)?;
                commands.iter().cloned().map(Attr::Custom).collect()
            }
        })
    }
}

impl Eval<crate::model::Attributes> for AttributeList {
    fn eval(&self, context: &mut Context) -> EvalResult<crate::model::Attributes> {
        Ok(Attributes(self.iter().try_fold(
            Vec::new(),
            |mut attributes, attribute| -> EvalResult<_> {
                attributes.append(&mut attribute.eval(context)?);
                Ok(attributes)
            },
        )?))
    }
}

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

use microcad_core::{Color, RenderResolution, Size2D, theme::Theme};
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
                AttributeCommand::Call(None, Some(argument_list)) => {
                    let argument_value_list: ArgumentValueList = argument_list.eval(context)?;
                    arguments.append(&mut argument_value_list.iter().cloned().collect())
                }
                AttributeCommand::Expression(expression) => arguments.push((
                    Identifier::default(),
                    ArgumentValue::new(expression.eval(context)?, expression.src_ref()),
                )),
                _ => unimplemented!(),
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

impl Eval<Option<MeasureCommand>> for syntax::Attribute {
    fn eval(&self, _: &mut Context) -> EvalResult<Option<MeasureCommand>> {
        todo!("MeasureCommand")
    }
}

impl Eval<Option<CustomCommand>> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<CustomCommand>> {
        match context.exporters().exporter_by_id(self.id.id()) {
            Ok(exporter) => {
                let arguments: ArgumentValueList = self.eval(context)?;
                match ArgumentMatch::find_match(&arguments, &exporter.parameters()) {
                    Ok(tuple) => Ok(Some(CustomCommand {
                        id: self.id.clone(),
                        arguments: Box::new(tuple),
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

impl Eval<Option<Size2D>> for syntax::AttributeCommand {
    fn eval(&self, _: &mut Context) -> EvalResult<Option<Size2D>> {
        todo!("Get Size2D, e.g. `size = (width = 10mm, height = 10mm) from AttributeCommand")
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
eval_to_attribute!(size: Size2D);

impl Eval<Option<crate::model::Attribute>> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<crate::model::Attribute>> {
        let id = self.id.id().as_str();
        use crate::model::Attribute as Attr;
        Ok(match id {
            "color" => {
                let color: Option<Color> = self.eval(context)?;
                color.map(Attr::Color)
            }
            "resolution" => {
                let resolution: Option<ResolutionAttribute> = self.eval(context)?;
                resolution.map(Attr::Resolution)
            }
            "theme" => {
                let theme: Option<std::rc::Rc<Theme>> = self.eval(context)?;
                theme.map(Attr::Theme)
            }
            "size" => {
                let size: Option<Size2D> = self.eval(context)?;
                size.map(Attr::Size)
            }
            "export" => {
                let export: Option<ExportCommand> = self.eval(context)?;
                export.map(Attr::Export)
            }
            "measure" => {
                let measure: Option<MeasureCommand> = self.eval(context)?;
                measure.map(Attr::Measure)
            }
            _ => {
                let custom_command: Option<CustomCommand> = self.eval(context)?;
                custom_command.map(Attr::Custom)
            }
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

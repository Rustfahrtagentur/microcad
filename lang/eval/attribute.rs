// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    Id,
    builtin::ExporterAccess,
    eval::{self, *},
    model_tree::{self, *},
    parameter,
    syntax::{self, *},
};

use microcad_core::{Color, RenderResolution};
use thiserror::Error;

/// Error type for attributes.
#[derive(Debug, Error)]
pub enum AttributeError {
    /// Unknown attribute.
    #[error("Attribute not supported: {0}")]
    NotSupported(QualifiedName),

    /// The attribute expected a different type.
    #[error("Expected one of types `{0}` for attribute `{1}`, got `{2}`")]
    ExpectedType(TypeList, QualifiedName, Type),

    /// Attribute cannot be assigned to an expression.
    #[error("Cannot assign attribute to expression `{0}`")]
    CannotAssignToExpression(Box<Expression>),

    /// Warning when an attribute has already been set.
    #[error("The attribute is already set: {0} = {1}")]
    AttributeAlreadySet(Identifier, Value),

    /// The attribute was not found.
    #[error("Not found: {0}")]
    NotFound(Identifier),
}

impl From<ArgumentMap> for Tuple {
    fn from(argument_map: ArgumentMap) -> Self {
        Tuple::new_named(
            argument_map
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        )
    }
}

impl Eval<Option<model_tree::ExportAttribute>> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<model_tree::ExportAttribute>> {
        match self {
            syntax::Attribute::Tuple(id, argument_list) if id.id() == "export" => {
                match ArgumentMap::find_match(
                    &argument_list.eval(context)?,
                    &vec![
                        parameter!(filename: String),
                        parameter!(resolution: Length = 0.1 /*mm*/),
                        parameter!(id: String = String::new()),
                    ]
                    .into(),
                ) {
                    Ok(argument_map) => {
                        let filename: std::path::PathBuf =
                            argument_map.get::<String>("filename").into();
                        let id: Id = argument_map.get::<String>("id").into();
                        let id: Option<Id> = if id.is_empty() { None } else { Some(id) };
                        let resolution = RenderResolution::new(
                            argument_map.get::<&Value>("resolution").try_scalar()?,
                        );

                        match context.find_exporter(&filename, &id) {
                            Ok(exporter) => {
                                return Ok(Some(ExportAttribute {
                                    filename,
                                    exporter,
                                    resolution,
                                }));
                            }
                            Err(err) => context.warning(self, err)?,
                        }
                    }
                    Err(err) => context.warning(self, err)?,
                }
            }
            _ => unreachable!(),
        }

        Ok(None)
    }
}

impl Eval<Option<model_tree::Attribute>> for syntax::Attribute {
    fn eval(&self, context: &mut Context) -> EvalResult<Option<model_tree::Attribute>> {
        match self {
            syntax::Attribute::Tuple(id, argument_list) => {
                let name = id.id().as_str();
                match name {
                    // Parse export attribute `export("test.svg")`
                    "export" => {
                        if let Some(attr) =
                            eval::Eval::<Option<ExportAttribute>>::eval(self, context)?
                        {
                            return Ok(Some(model_tree::Attribute::Export(attr)));
                        }
                    }
                    // Parse exporter specific attribute, e.g. `svg(style = "fill:none")`
                    _ => match context.exporter_by_id(id.id()) {
                        Ok(exporter) => {
                            match ArgumentMap::find_match(
                                &argument_list.eval(context)?,
                                &exporter.parameters(),
                            ) {
                                Ok(args) => {
                                    return Ok(Some(model_tree::Attribute::ExporterSpecific(
                                        id.clone(),
                                        args,
                                    )));
                                }
                                Err(err) => {
                                    context.warning(self, err)?;
                                }
                            }
                        }
                        Err(err) => {
                            context.warning(id, err)?;
                        }
                    },
                }
            }
            syntax::Attribute::NameValue(id, expression) => {
                let name = id.id().as_str();
                let value = expression.eval(context)?;

                match (name, value) {
                    ("color", Value::Tuple(tuple)) => match tuple.as_ref().try_into() {
                        Ok(color) => return Ok(Some(model_tree::Attribute::Color(color))),
                        Err(err) => context.warning(self, err)?,
                    },
                    ("color", Value::String(string)) => match string.parse::<Color>() {
                        Ok(color) => return Ok(Some(model_tree::Attribute::Color(color))),
                        Err(err) => context.warning(self, err)?,
                    },
                    ("resolution", value) => match value.try_into() {
                        Ok(resolution_attribute) => {
                            return Ok(Some(model_tree::Attribute::Resolution(
                                resolution_attribute,
                            )));
                        }
                        Err(err) => context.warning(self, err)?,
                    },
                    _ => {}
                }
            }
        }

        Ok(None)
    }
}

impl Eval<Attributes> for AttributeList {
    fn eval(&self, context: &mut Context) -> EvalResult<Attributes> {
        let mut attributes = Vec::new();

        for attribute in self.iter() {
            if let Some(attribute) = attribute.eval(context)? {
                attributes.push(attribute);
            }
        }

        Ok(model_tree::Attributes::new(attributes))
    }
}

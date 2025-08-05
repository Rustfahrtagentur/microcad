// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI export command

use anyhow::anyhow;
use microcad_builtin::{Exporter, ExporterAccess, ExporterRegistry};
use microcad_core::RenderResolution;
use microcad_lang::{model::*, ty::QuantityType, value::*};

use crate::{config::Config, *};

/// Parse and evaluate and export a µcad file.
#[derive(clap::Parser)]
pub struct ExportArgs {
    /// Input µcad file.
    pub input: std::path::PathBuf,

    /// Output file (e.g. an SVG or STL).
    pub output: Option<std::path::PathBuf>,

    /// List all export target files.
    #[arg(short = 'l', long = "list", action = clap::ArgAction::SetTrue)]
    pub list: bool,

    /// The resolution of this export.
    ///
    /// The resolution can changed relatively `200%` or to an absolute value `0.05mm`.
    #[arg(short = 'r', long = "resolution", default_value = "0.1mm")]
    pub resolution: String,

    /// Layers to export (or all layers).
    #[arg(long = "layer", action = clap::ArgAction::Append, default_value = "./lib", global = true)]
    pub layers: Vec<String>,
}

impl ExportArgs {
    /// Get default exporter.
    fn default_exporter(
        output_type: &OutputType,
        config: &Config,
        exporters: &ExporterRegistry,
    ) -> anyhow::Result<std::rc::Rc<dyn Exporter>> {
        match output_type {
            OutputType::NotDetermined => Err(anyhow!("Could not determine output type.")),
            OutputType::Geometry2D => {
                Ok(exporters.exporter_by_id(&(&config.export.sketch).into())?)
            }
            OutputType::Geometry3D => Ok(exporters.exporter_by_id(&(&config.export.part).into())?),
            OutputType::InvalidMixed => Err(anyhow!(
                "Invalid output type, the model cannot be exported."
            )),
        }
    }

    /// Parse render resolution.
    fn resolution(&self) -> RenderResolution {
        use microcad_lang::*;

        use std::str::FromStr;
        let value = syntax::NumberLiteral::from_str(&self.resolution)
            .map(|literal| literal.value())
            .unwrap_or(value::Value::None);

        match value {
            value::Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Length,
            }) => RenderResolution::new(value),
            _ => {
                let default = RenderResolution::default();
                log::warn!(
                    "Invalid resolution `{resolution}`. Using default resolution: {value}mm",
                    resolution = self.resolution,
                    value = default.linear
                );
                default
            }
        }
    }

    /// Get default export attribute.
    fn default_export_attribute(
        &self,
        model: &Model,
        config: &Config,
        exporters: &ExporterRegistry,
    ) -> anyhow::Result<ExportCommand> {
        let default_exporter =
            Self::default_exporter(&model.final_output_type(), config, exporters);
        let resolution = self.resolution();

        match &self.output {
            Some(filename) => Ok(ExportCommand {
                filename: filename.to_path_buf(),
                resolution,
                exporter: exporters
                    .exporter_by_filename(filename)
                    .or(default_exporter)?,
            }),
            None => {
                let mut filename = self.input.clone();
                let exporter = default_exporter?;

                let ext = exporter
                    .file_extensions()
                    .first()
                    .unwrap_or(&exporter.id())
                    .to_string();
                filename.set_extension(&ext);

                Ok(ExportCommand {
                    filename,
                    exporter,
                    resolution,
                })
            }
        }
    }

    /// Get all models that are supposed to be exported.
    ///
    /// All child models of `model` that are in the same source file and
    /// that have an `export` attribute will be exported.
    ///
    /// If no models have been found, we simply export this model with the default export attribute.
    pub fn target_models(
        &self,
        model: &Model,
        config: &Config,
        exporters: &ExporterRegistry,
    ) -> anyhow::Result<Vec<(Model, ExportCommand)>> {
        let mut models = model
            .source_file_descendants()
            .fold(Vec::new(), |mut models, model| {
                let b = model.borrow();
                models.append(
                    &mut b
                        .attributes
                        .get_exports()
                        .iter()
                        .map(|attr| (model.clone(), attr.clone()))
                        .collect(),
                );
                models
            });

        // No models with export attributes have been found.
        if models.is_empty() {
            // Add the root model with default exporters.
            models.push((
                model.clone(),
                self.default_export_attribute(model, config, exporters)?,
            ))
        }

        Ok(models)
    }

    pub fn export_targets(&self, models: &[(Model, ExportCommand)]) -> anyhow::Result<()> {
        models
            .iter()
            .try_for_each(|(model, attr)| -> anyhow::Result<()> {
                let value = attr.export(model)?;
                if !matches!(value, Value::None) {
                    log::info!("{value}");
                };
                Ok(())
            })?;
        Ok(())
    }

    pub fn list_targets(&self, models: &Vec<(Model, ExportCommand)>) -> anyhow::Result<()> {
        for (model, attr) in models {
            log::info!("{model} => {attr}", model = model.signature());
        }
        Ok(())
    }
}

/// Parse and evaluate and export a µcad file.
#[derive(clap::Parser)]
pub struct Export {
    /// Input µcad file.
    #[clap(flatten)]
    args: ExportArgs,
}

impl RunCommand for Export {
    fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        let mut context = cli.make_context(&self.args.input)?;
        let model = context.eval().expect("Valid model");
        let config = cli.fetch_config()?;

        let target_models = &self
            .args
            .target_models(&model, &config, context.exporters())?;
        if self.args.list {
            self.args.list_targets(target_models)
        } else {
            self.args.export_targets(target_models)
        }
    }
}

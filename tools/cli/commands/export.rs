// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI export command

use anyhow::anyhow;
use microcad_builtin::{Exporter, ExporterAccess, ExporterRegistry};
use microcad_core::RenderResolution;
use microcad_lang::{model_tree::*, ty::QuantityType, value::*};

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
}

/// Parse and evaluate and export a µcad file.
#[derive(clap::Parser)]
pub struct Export {
    /// Input µcad file.
    #[clap(flatten)]
    args: ExportArgs,
}

impl Export {
    /// Get default exporter.
    fn default_exporter(
        output_type: &ModelNodeOutputType,
        config: &Config,
        exporters: &ExporterRegistry,
    ) -> anyhow::Result<std::rc::Rc<dyn Exporter>> {
        match output_type {
            ModelNodeOutputType::NotDetermined => {
                Err(anyhow!("Could not determine node output type."))
            }
            ModelNodeOutputType::Geometry2D => {
                Ok(exporters.exporter_by_id(&(&config.export.sketch).into())?)
            }
            ModelNodeOutputType::Geometry3D => {
                Ok(exporters.exporter_by_id(&(&config.export.part).into())?)
            }
            ModelNodeOutputType::InvalidMixed => Err(anyhow!(
                "Invalid node output type, the node cannot be exported."
            )),
        }
    }

    /// Parse render resolution.
    fn resolution(&self) -> RenderResolution {
        use microcad_lang::*;

        use std::str::FromStr;
        let value = syntax::NumberLiteral::from_str(&self.args.resolution)
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
                    resolution = self.args.resolution,
                    value = default.linear
                );
                default
            }
        }
    }

    /// Get default export attribute.
    fn default_export_attribute(
        &self,
        node: &ModelNode,
        config: &Config,
        exporters: &ExporterRegistry,
    ) -> anyhow::Result<ExportAttribute> {
        let default_exporter = Self::default_exporter(&node.output_type(), config, exporters);
        let resolution = self.resolution();

        match &self.args.output {
            Some(filename) => Ok(ExportAttribute {
                filename: filename.to_path_buf(),
                resolution,
                exporter: exporters
                    .exporter_by_filename(filename)
                    .or(default_exporter)?,
            }),
            None => {
                let mut filename = self.args.input.clone();
                let exporter = default_exporter?;

                let ext = exporter
                    .file_extensions()
                    .first()
                    .unwrap_or(&exporter.id())
                    .to_string();
                filename.set_extension(&ext);

                Ok(ExportAttribute {
                    filename,
                    exporter,
                    resolution,
                })
            }
        }
    }

    /// Get all nodes that are supposed to be exported.
    ///
    /// All child nodes of `node` that are in the same source file and
    /// that have an `export` attribute will be exported.
    ///
    /// If no nodes have been found, we simply export this node with the default export attribute.
    fn target_nodes(
        &self,
        node: &ModelNode,
        config: &Config,
        exporters: &ExporterRegistry,
    ) -> anyhow::Result<Vec<(ModelNode, ExportAttribute)>> {
        let mut nodes: Vec<(ModelNode, ExportAttribute)> = node
            .source_file_descendants()
            .filter_map(|node| {
                let b = node.borrow();
                b.attributes()
                    .get_export_attribute()
                    .map(|attr| (node.clone(), attr))
            })
            .collect();

        // No nodes with export attributes have been found.
        if nodes.is_empty() {
            // Add the root node with default exporters.
            nodes.push((
                node.clone(),
                self.default_export_attribute(node, config, exporters)?,
            ))
        }

        Ok(nodes)
    }

    fn export_targets(&self, nodes: &[(ModelNode, ExportAttribute)]) -> anyhow::Result<()> {
        nodes
            .iter()
            .try_for_each(|(node, attr)| -> anyhow::Result<()> {
                let value = attr.export(node)?;
                if !matches!(value, Value::None) {
                    log::info!("{value}");
                };
                Ok(())
            })?;
        Ok(())
    }

    fn list_targets(&self, nodes: &Vec<(ModelNode, ExportAttribute)>) -> anyhow::Result<()> {
        for (node, attr) in nodes {
            log::info!("{node} => {attr}", node = node.signature());
        }
        Ok(())
    }
}

impl RunCommand for Export {
    fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        let mut context = cli.make_context(&self.args.input)?;
        let node = context.eval().expect("Valid node");
        let config = cli.fetch_config()?;

        let target_nodes = &self.target_nodes(&node, &config, context.exporters())?;
        if self.args.list {
            self.list_targets(target_nodes)
        } else {
            self.export_targets(target_nodes)
        }
    }
}

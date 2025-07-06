// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI export command

use microcad_lang::model_tree::{ExportAttribute, GetAttribute, ModelNode};

use crate::*;

/// Parse and evaluate and export a µcad file.
#[derive(clap::Parser)]
pub struct Export {
    /// Input µcad file.
    input: std::path::PathBuf,

    /// Output file (e.g. an SVG or STL).
    output: Option<std::path::PathBuf>,

    /// List all export target files.
    #[arg(short = 'l', long = "list", action = clap::ArgAction::SetTrue)]
    list: bool,

    /// Export a specific target.
    #[arg(short = 't', long = "target", action = clap::ArgAction::Append)]
    target: Vec<String>,

    /// The resolution of this export.
    ///
    /// The resolution can changed relatively `200%` or to an absolute value `0.05mm`.
    #[arg(short = 'r', long = "resolution", default_value = "0.1mm")]
    resolution: String,
}

impl Export {
    fn _target_name(_node: &ModelNode) -> Option<String> {
        todo!()
    }

    fn target_nodes(&self, node: &ModelNode) -> Vec<(ModelNode, ExportAttribute)> {
        let nodes: Vec<(ModelNode, ExportAttribute)> = node
            .source_file_descendants()
            .filter_map(|node| {
                let b = node.borrow();
                b.attributes()
                    .get_export_attribute()
                    .map(|attr| (node.clone(), attr))
            })
            .collect();

        nodes
    }

    fn export_targets(&self, nodes: &Vec<(ModelNode, ExportAttribute)>) -> anyhow::Result<()> {
        for (node, attr) in nodes {
            attr.exporter.export(node, &attr.filename)?;
        }

        Ok(())
    }

    fn list_targets(&self, nodes: &Vec<(ModelNode, ExportAttribute)>) -> anyhow::Result<()> {
        for (node, attr) in nodes {
            println!("{node} => {attr}", node = node.signature());
        }
        Ok(())
    }
}

impl RunCommand for Export {
    fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        let mut context = cli.make_context(&self.input)?;
        let node = context.eval().expect("Valid node");

        let target_nodes = &self.target_nodes(&node);
        if self.list {
            self.list_targets(target_nodes)
        } else {
            self.export_targets(target_nodes)
        }
    }
}

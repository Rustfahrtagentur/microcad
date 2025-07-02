// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI export command

use microcad_lang::model_tree::{ModelNode, ModelNodes};

use crate::commands::RunCommand;

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
}

impl Export {
    fn _target_name(_node: &ModelNode) -> Option<String> {
        todo!()
    }

    fn target_nodes(_node: &ModelNode) -> ModelNodes {
        todo!()
    }

    fn export_targets(&self, _nodes: &ModelNodes) -> anyhow::Result<()> {
        todo!()
    }

    fn list_targets(&self, _nodes: &ModelNodes) -> anyhow::Result<()> {
        todo!()
    }
}

impl RunCommand for Export {
    fn run(&self, cli: &crate::cli::Cli) -> anyhow::Result<()> {
        let mut context = cli.make_context(&self.input)?;
        let node = context.eval().expect("Valid node");

        let target_nodes = &Self::target_nodes(&node);
        if self.list {
            self.list_targets(target_nodes)
        } else {
            self.export_targets(target_nodes)
        }
    }
}

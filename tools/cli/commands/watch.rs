// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI parse command

use crate::{commands::export::ExportArgs, *};

#[derive(clap::Parser)]
pub struct Watch {
    /// Export arguments.
    #[clap(flatten)]
    pub export_args: ExportArgs,
}

/// Run this command for a CLI.
impl RunCommand for Watch {
    fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        let mut watcher = Watcher::new()?;
        let config = cli.fetch_config()?;
        let export = &self.export_args;

        if export.list {
            let mut context = cli.make_context(&self.export_args.input)?;
            let node = context.eval().expect("Valid node");
            export.list_targets(&export.target_nodes(&node, &config, context.exporters())?)
        } else {
            // Recompile whenever something relevant happens.
            loop {
                let mut context = cli.make_context(&self.export_args.input)?;
                // Re-evaluate context.
                if let Ok(node) = context.eval() {
                    let target_nodes = &export.target_nodes(&node, &config, context.exporters())?;

                    export.export_targets(target_nodes)?;
                }

                // Watch all dependencies of the most recent compilation.
                watcher.update(vec![self.export_args.input.clone()])?;

                // Wait until anything relevant happens.
                watcher.wait()?;
            }
        }
    }
}

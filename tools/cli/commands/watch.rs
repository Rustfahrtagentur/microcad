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
            // run prior parse step
            let (context, model) = Eval {
                input: self.export_args.input.clone(),
                verbose: false,
            }
            .run(cli)?;
            export.list_targets(&export.target_models(&model, &config, context.exporters())?)
        } else {
            // Recompile whenever something relevant happens.
            loop {
                // run prior parse step
                let result = Eval {
                    input: self.export_args.input.clone(),
                    verbose: false,
                }
                .run(cli);

                match result {
                    Ok((context, model)) => {
                        match export.target_models(&model, &config, context.exporters()) {
                            Ok(target_models) => export.export_targets(&target_models)?,
                            Err(err) => log::error!("{err}"),
                        }
                    }
                    Err(err) => {
                        log::error!("{err}");
                    }
                }

                // Watch all dependencies of the most recent compilation.
                watcher.update(vec![self.export_args.input.clone()])?;

                // Wait until anything relevant happens.
                watcher.wait()?;
            }
        }
    }
}

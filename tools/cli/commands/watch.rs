// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI parse command

use crate::*;

#[derive(clap::Parser)]
pub struct Watch {
    /// Export arguments.
    #[clap(flatten)]
    pub export: Export,
}

/// Run this command for a CLI.
impl RunCommand for Watch {
    fn run(&self, cli: &Cli) -> anyhow::Result<()> {
        let mut watcher = Watcher::new()?;

        if !self.export.dry_run {
            // Recompile whenever something relevant happens.
            loop {
                // run prior parse step
                match self.export.run(cli) {
                    Ok(target_models) => self.export.export_targets(&target_models)?,
                    Err(err) => log::error!("{err}"),
                }

                // Watch all dependencies of the most recent compilation.
                watcher.update(vec![self.export.eval.resolve.parse.input.clone()])?;

                // Wait until anything relevant happens.
                watcher.wait()?;
            }
        }

        Ok(())
    }
}

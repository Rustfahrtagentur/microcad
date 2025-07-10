// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
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
    fn run(&self, _: &crate::cli::Cli) -> anyhow::Result<()> {
        println!("watch: {:?}", self.export_args.input);
        Ok(())
    }
}

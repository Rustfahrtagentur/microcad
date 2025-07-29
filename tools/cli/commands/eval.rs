// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI eval commands

use microcad_lang::diag::{Diag, WriteToFile};

use crate::commands::RunCommand;

#[derive(clap::Parser)]
pub struct Eval {
    /// Input µcad file.
    pub input: std::path::PathBuf,
    /// Output models.
    pub output: Option<std::path::PathBuf>,
}

impl RunCommand for Eval {
    fn run(&self, cli: &crate::cli::Cli) -> anyhow::Result<()> {
        let mut context = cli.make_context(&self.input)?;
        let model = context.eval().expect("Valid model");

        log::info!("Result:");
        match context.has_errors() {
            true => {
                log::warn!("Evaluated with errors:");
                eprintln!("{}", context.diagnosis());
            }
            false => log::info!("Evaluated successfully!"),
        }

        match &self.output {
            Some(filename) => model.write_to_file(&filename)?,
            None => println!("{model}"),
        }

        Ok(())
    }
}

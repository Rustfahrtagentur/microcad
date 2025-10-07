// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI eval commands

use microcad_lang::{diag::*, eval::*, model::Model, tree_display::*};

use crate::commands::{Resolve, RunCommand};
use anyhow::*;

#[derive(clap::Parser)]
pub struct Eval {
    /// Input µcad file.
    pub input: std::path::PathBuf,

    /// Write resolve context to stdout
    #[clap(short, long, default_value = "true")]
    pub verbose: bool,
}

impl RunCommand<(EvalContext, Model)> for Eval {
    fn run(&self, cli: &crate::cli::Cli) -> anyhow::Result<(EvalContext, Model)> {
        if !cli.has_std_lib() {
            return Err(anyhow!(
                "No std library was found. Use `microcad install std` to install the std library."
            ));
        }
        // run prior parse step
        let resolve_context = Resolve {
            input: self.input.clone(),
            verbose: false,
            check: false,
        }
        .run(cli)?;

        let mut context = EvalContext::new(resolve_context, Stdout::new());

        let model = context.eval().expect("Valid model");

        log::info!("Result:");
        match context.has_errors() {
            true => {
                log::warn!("Evaluated with errors:");
                eprintln!("{}", context.diagnosis());
            }
            false => log::info!("Evaluated successfully!"),
        }

        if self.verbose {
            println!("{}", FormatTree(&model))
        }

        Ok((context, model))
    }
}

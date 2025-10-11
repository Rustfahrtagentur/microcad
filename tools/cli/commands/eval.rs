// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI eval commands

use microcad_lang::{diag::*, eval::*, model::Model, tree_display::*};

use crate::commands::{Resolve, RunCommand};
use anyhow::*;

#[derive(clap::Parser)]
pub struct Eval {
    #[clap(flatten)]
    pub resolve: Resolve,

    /// Print model tree.
    #[clap(long)]
    pub model: bool,
}

impl RunCommand<(EvalContext, Option<Model>)> for Eval {
    fn run(&self, cli: &crate::cli::Cli) -> anyhow::Result<(EvalContext, Option<Model>)> {
        if !cli.has_std_lib() {
            return Err(anyhow!(
                "No std library was found. Use `microcad install std` to install the std library."
            ));
        }
        // run prior parse step
        let resolve_context = self.resolve.run(cli)?;

        let mut context = EvalContext::new(
            resolve_context,
            Stdout::new(),
            microcad_builtin::builtin_exporters(),
            microcad_builtin::builtin_importers(),
        );

        log::info!("Result:");
        match context.has_errors() {
            true => {
                log::warn!("Evaluated with errors:");
                eprintln!("{}", context.diagnosis());
            }
            false => log::info!("Evaluated successfully!"),
        }

        if self.model {
            match context.eval() {
                Result::Ok(Some(model)) => {
                    println!("{}", FormatTree(&model));
                    Ok((context, Some(model)))
                }
                Result::Ok(None) => {
                    eprintln!("No output model.");
                    Ok((context, None))
                }
                Err(err) => {
                    eprintln!("Model construction failed.");
                    Ok(Err(err)?)
                }
            }
        } else {
            Ok((context, None))
        }
    }
}

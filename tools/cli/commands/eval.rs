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
        if !self.resolve.has_std_lib() {
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

        let result = context.eval();

        match context.has_errors() {
            true => {
                eprintln!("Evaluation failed:");
                eprintln!("{}", context.diagnosis());
            }
            false => log::info!("Evaluated successfully!"),
        }

        match result {
            Result::Ok(Some(model)) => {
                if cli.is_eval() {
                    eprintln!("Created model!");
                }
                if self.model {
                    println!("{}", FormatTree(&model));
                }
                Ok((context, Some(model)))
            }
            Result::Ok(None) => {
                if cli.is_eval() {
                    eprintln!("No output model!");
                }
                Ok((context, None))
            }
            Err(err) => {
                if cli.is_eval() {
                    eprintln!("Model construction failed.");
                }
                Ok(Err(err)?)
            }
        }
    }
}

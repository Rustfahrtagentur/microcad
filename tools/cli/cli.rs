// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI.

use clap::Parser;
use microcad_lang::{eval::Context, parse::ParseResult};

use crate::commands::{self, Commands, RunCommand};

/// µcad cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// display processing time
    #[arg(short, long, default_value = "false", global = true)]
    time: bool,

    /// Paths to search for files.
    #[arg(short = 'p', long = "search-path", action = clap::ArgAction::Append, default_value = "./lib", global = true)]
    search_paths: Vec<std::path::PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    /// Run the CLI.
    pub fn run(&self) -> anyhow::Result<()> {
        let start = std::time::Instant::now();

        match &self.command {
            Commands::Parse(parse) => parse.run(self)?,
            Commands::Resolve(resolve) => resolve.run(self)?,
            Commands::Eval(eval) => eval.run(self)?,
            Commands::Export(export) => export.run(self)?,
            Commands::Create(create) => create.run(self)?,
        }

        if self.time {
            log::info!("Processing Time: {:?}", start.elapsed());
        }
        Ok(())
    }

    /// Make a new context from an input file.
    pub fn make_context(&self, input: impl AsRef<std::path::Path>) -> ParseResult<Context> {
        Ok(microcad_builtin::builtin_context(
            commands::Resolve {
                input: input.as_ref().to_path_buf(),
            }
            .resolve()?,
            &self.search_paths,
        ))
    }
}

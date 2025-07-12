// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI.

use clap::Parser;
use microcad_lang::{eval::Context, parse::ParseResult};

use crate::commands::*;
use crate::config::Config;

/// µcad cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Display processing time.
    #[arg(short = 'T', long, default_value = "false", action = clap::ArgAction::SetTrue)]
    time: bool,

    /// Paths to search for files.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append, default_value = "./lib", global = true)]
    search_paths: Vec<std::path::PathBuf>,

    /// Load config from file.
    #[arg(short = 'C', long = "config")]
    config: Option<std::path::PathBuf>,

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
            Commands::Watch(watch) => watch.run(self)?,
        }

        if self.time {
            log::info!("Processing Time: {:?}", start.elapsed());
        }
        Ok(())
    }

    /// Make a new context from an input file.
    pub fn make_context(&self, input: impl AsRef<std::path::Path>) -> ParseResult<Context> {
        Ok(microcad_builtin::builtin_context(
            crate::commands::Resolve {
                input: input.as_ref().to_path_buf(),
                output: None,
            }
            .resolve()?,
            &self.search_paths,
        ))
    }

    /// Fetch a config from file or default config.
    pub fn fetch_config(&self) -> anyhow::Result<Config> {
        match &self.config {
            Some(config) => Config::load(config),
            None => Ok(Config::default()),
        }
    }
}

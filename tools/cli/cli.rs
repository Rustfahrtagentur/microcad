// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI.

use anyhow::anyhow;
use clap::Parser;
use microcad_lang::{eval::Context, resolve::*};

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
    ///
    /// Uses `~/.microcad/lib`
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append, global = true)]
    pub search_paths: Vec<std::path::PathBuf>,

    /// Load config from file.
    #[arg(short = 'C', long = "config")]
    config: Option<std::path::PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    /// Create a new CLI with default search paths.
    ///
    /// Also checks if there is a µcad std library installed and returns an error in case the library has not been found.
    pub fn new() -> anyhow::Result<Self> {
        let mut cli: Self = Self::parse();
        cli.search_paths.append(&mut Self::default_search_paths());

        if !cli.has_std_lib() {
            Err(anyhow!(
                "No std library was found. Use `microcad install std` to install the std library."
            ))
        } else {
            Ok(cli)
        }
    }

    /// `./lib` (if exists) and `~/.microcad/lib`.
    pub fn default_search_paths() -> Vec<std::path::PathBuf> {
        let local_dir = std::path::PathBuf::from("./lib");
        let mut search_paths = Vec::new();

        if let Some(global_root_dir) = Self::global_root_dir() {
            search_paths.push(global_root_dir);
        }
        if local_dir.exists() {
            search_paths.push(local_dir);
        }

        search_paths
    }

    /// Returns global root dir `~/.microcad/lib`, if it exists.
    pub fn global_root_dir() -> Option<std::path::PathBuf> {
        if let Some(home_dir) = home::home_dir() {
            let global_root_dir = home_dir.join(".microcad/lib");
            if global_root_dir.exists() {
                return Some(home_dir.join(".microcad/lib"));
            }
        }

        None
    }

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
            Commands::Install(install) => install.run(self)?,
        }

        if self.time {
            log::info!("Processing Time: {:?}", start.elapsed());
        }
        Ok(())
    }

    /// Make a new context from an input file.
    pub fn make_context(&self, input: impl AsRef<std::path::Path>) -> ResolveResult<Context> {
        microcad_builtin::builtin_context(
            crate::commands::Resolve {
                input: input.as_ref().to_path_buf(),
                output: None,
            }
            .load()?,
            &self.search_paths,
        )
    }

    /// Fetch a config from file or default config.
    pub fn fetch_config(&self) -> anyhow::Result<Config> {
        match &self.config {
            Some(config) => Config::load(config),
            None => Ok(Config::default()),
        }
    }

    /// Check if we have a std lib in search paths.
    pub fn has_std_lib(&self) -> bool {
        self.search_paths.iter().any(|dir| {
            let file_path = dir.join("std/mod.µcad");
            file_path.exists() && file_path.is_file()
        })
    }
}

// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI.

use clap::Parser;

use crate::commands::*;
use crate::config::Config;

/// µcad cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Display processing time.
    #[arg(short = 'T', long, default_value = "false", action = clap::ArgAction::SetTrue)]
    pub(crate) time: bool,

    /// Load config from file.
    #[arg(short = 'C', long)]
    config: Option<std::path::PathBuf>,

    /// Verbosity level (use -v, -vv, or -vvv)
    #[arg(short, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,

    #[command(subcommand)]
    command: Commands,
}

impl Default for Cli {
    /// Create a new CLI with default search paths.
    fn default() -> Self {
        Self::parse()
    }
}

impl Cli {
    /// `./lib` (if exists) and `~/.config/microcad/lib` (if exists).
    pub fn default_search_paths() -> Vec<std::path::PathBuf> {
        let local_dir = std::path::PathBuf::from("./lib");
        let mut search_paths = Vec::new();

        if let Some(global_root_dir) = Self::global_root_dir() {
            if global_root_dir.exists() {
                search_paths.push(global_root_dir);
            }
        }
        if local_dir.exists() {
            search_paths.push(local_dir);
        }

        search_paths
    }

    /// Returns microcad's config dir, even if it does not exist.
    ///
    /// On Linux, the config dir is located in `~/.config/microcad`.
    pub fn config_dir() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|dir| dir.join("microcad"))
    }

    /// Returns global root dir, even if it does not exist.
    ///
    /// On Linux, the root dir is located in `~/.config/microcad/lib`.
    pub fn global_root_dir() -> Option<std::path::PathBuf> {
        Self::config_dir().map(|dir| dir.join("lib"))
    }

    /// Run the CLI.
    pub fn run(&self) -> anyhow::Result<()> {
        let start = std::time::Instant::now();

        match &self.command {
            Commands::Parse(parse) => {
                parse.run(self)?;
            }
            Commands::Resolve(resolve) => {
                resolve.run(self)?;
            }
            Commands::Eval(eval) => {
                eval.run(self)?;
            }
            Commands::Export(export) => {
                export.run(self)?;
            }
            Commands::Create(create) => {
                create.run(self)?;
            }
            Commands::Watch(watch) => {
                watch.run(self)?;
            }
            Commands::Install(install) => {
                install.run(self)?;
            }
        }

        if self.time {
            eprintln!(
                "Overall Time   : {}",
                Self::time_to_string(&start.elapsed())
            );
        }
        Ok(())
    }

    /// Fetch a config from file or default config.
    pub fn fetch_config(&self) -> anyhow::Result<Config> {
        match &self.config {
            Some(config) => Config::load(config),
            None => Ok(Config::default()),
        }
    }

    pub(super) fn is_parse(&self) -> bool {
        matches!(self.command, Commands::Parse(..))
    }

    pub(super) fn is_resolve(&self) -> bool {
        matches!(self.command, Commands::Resolve(..))
    }

    pub(super) fn is_eval(&self) -> bool {
        matches!(self.command, Commands::Eval(..))
    }

    pub(super) fn is_export(&self) -> bool {
        matches!(self.command, Commands::Export(..))
    }

    pub(super) fn time_to_string(duration: &std::time::Duration) -> String {
        use num_format::{Locale, ToFormattedString};
        format!(
            "{:>8}µs",
            duration.as_micros().to_formatted_string(&Locale::en)
        )
    }
}

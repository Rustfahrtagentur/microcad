// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad command line interpreter

extern crate clap;
extern crate microcad_lang;

mod cli;
mod commands;
mod config;
pub mod watcher;

pub use cli::*;
use commands::*;

pub use watcher::*;

/// Main of the command line interpreter
fn main() -> anyhow::Result<()> {
    let cli = Cli::default();

    // Initialize env_logger with a default filter level
    env_logger::Builder::from_default_env()
        .filter_level(match cli.verbose {
            0 => log::LevelFilter::Off,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            3 => log::LevelFilter::Trace,
            _ => panic!("unknown verbosity level"),
        }) // Set the default log level
        .init();

    cli.run()
}

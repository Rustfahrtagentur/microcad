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
    env_logger::init();
    let cli = Cli::new();

    cli.run()
}

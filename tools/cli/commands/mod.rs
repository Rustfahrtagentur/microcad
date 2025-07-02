// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI commands

mod create;
mod eval;
mod export;
mod parse;
mod resolve;

use clap::Subcommand;

pub use create::Create;
pub use eval::Eval;
pub use export::Export;
pub use parse::Parse;
pub use resolve::Resolve;

#[derive(Subcommand)]
pub enum Commands {
    /// Parse a µcad file.
    Parse(Parse),

    /// Parse and resolve a µcad file.
    Resolve(Resolve),

    /// Parse and evaluate a µcad file.
    Eval(Eval),

    /// Parse and evaluate and export a µcad file.
    Export(Export),

    /// Create a new source file with µcad extension.
    Create(Create),
}

/// Run this command for a cli
pub trait RunCommand {
    fn run(&self, cli: &crate::cli::Cli) -> anyhow::Result<()>;
}

// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD command line interpreter

#![warn(missing_docs)]

extern crate clap;

extern crate microcad_lang;

use clap::{Parser, Subcommand};
use microcad_lang::parse::SourceFile;

/// µCAD cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse the µCAD file
    Parse {
        /// Input µCAD file
        input: String,
    },

    /// Evaluates the µCAD file
    Eval {
        /// Input µCAD file
        input: String,
    },

    /// Exports the µCAD file
    Export {
        /// Input µCAD file
        input: String,
    },
}

/// Main of the command line interpreter
fn main() {
    let cli = Cli::parse();

    if let Err(err) = run(cli.command) {
        eprintln!("{err}")
    }
}

fn run(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::Parse { input } => {
            parse(&input)?;
        }
        Commands::Eval { input } => {
            eval(&input)?;
        }
        Commands::Export { input } => export(&input)?,
    }

    Ok(())
}

fn parse(input: &str) -> anyhow::Result<SourceFile> {
    let source_file = SourceFile::load(input)?;

    eprintln!("{input} parsed successfully");
    Ok(source_file)
}

fn eval(input: &str) -> anyhow::Result<microcad_lang::ObjectNode> {
    let source_file = parse(input)?;
    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_std("std")
        .build();

    let node = context.eval().map_err(|err| anyhow::anyhow!("{err}"))?;

    if context.diag().has_errors() {
        let mut w = std::io::stderr();
        context.diag().pretty_print(&mut w, &context)?;
    } else {
        eprintln!("{input} evaluated successfully");
    }

    Ok(node)
}

fn export(input: &str) -> anyhow::Result<()> {
    let node = eval(input)?;
    microcad_std::export(node)?;
    Ok(())
}

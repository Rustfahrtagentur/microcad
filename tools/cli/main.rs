// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad command line interpreter

extern crate clap;

extern crate microcad_lang;

use std::path::Path;

use clap::{Parser, Subcommand};
use microcad_lang::{parse::SourceFile, ObjectNode};

/// µcad cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Standard library search path
    #[arg(long, default_value = "std")]
    std: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse the µcad file
    Parse {
        /// Input µcad file
        input: String,
    },

    /// Evaluates the µcad file
    Eval {
        /// Input µcad file
        input: String,
    },

    /// Exports the µcad file
    Export {
        /// Input µcad file
        input: String,
    },
}

/// Main of the command line interpreter
fn main() {
    let cli = Cli::parse();

    if let Err(err) = run(&cli) {
        eprintln!("{err}")
    }
}

fn run(cli: &Cli) -> anyhow::Result<()> {
    match &cli.command {
        Commands::Parse { input } => {
            parse(&input)?;
        }
        Commands::Eval { input } => {
            eval(parse(&input)?, &cli.std)?;
        }
        Commands::Export { input } => {
            export(eval(parse(&input)?, &cli.std)?)?;
        }
    }
    eprintln!("Success!");

    Ok(())
}

fn parse(input: impl AsRef<Path>) -> anyhow::Result<SourceFile> {
    Ok(SourceFile::load(input)?)
}

fn eval(source_file: SourceFile, std: impl AsRef<Path>) -> anyhow::Result<ObjectNode> {
    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_std(std)?
        .build();

    let node = context.eval().map_err(|err| anyhow::anyhow!("{err}"))?;

    if context.diag().has_errors() {
        let mut w = std::io::stderr();
        context.diag().pretty_print(&mut w, &context)?;
    }

    Ok(node)
}

fn export(node: ObjectNode) -> anyhow::Result<()> {
    Ok(microcad_std::export(node)?)
}

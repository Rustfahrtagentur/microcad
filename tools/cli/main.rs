// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD command line interpreter

#![warn(missing_docs)]

extern crate clap;

extern crate microcad_lang;

use clap::Parser;
use microcad_lang::parse::SourceFile;

/// µCAD cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input µCAD file
    input: String,
}

/// Main of the command line interpreter
fn main() {
    let cli = Cli::parse();

    if let Err(err) = load(&cli.input) {
        eprintln!("{err}");
    }
}

fn load(filename: &str) -> anyhow::Result<()> {
    let source_file = SourceFile::load(filename)?;
    let mut context = microcad_std::ContextBuilder::new(source_file)
        .with_std("std")
        .build();

    let node = context.eval().map_err(|err| anyhow::anyhow!("{err}"))?;
    let mut w = std::io::stdout();
    context.diag().pretty_print(&mut w, &context)?;

    microcad_std::export(node)?;

    Ok(())
}

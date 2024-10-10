// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD command line interpreter

#![warn(missing_docs)]

extern crate clap;

extern crate microcad_lang;

use clap::Parser;
use microcad_lang::parse::SourceFile;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input µCAD file
    input: Option<String>,
}

/// Main of the command line interpreter
fn main() {
    let cli = Cli::parse();

    match cli {
        Cli { input: Some(input) } => {
            let source_file = SourceFile::load(input).unwrap();
            let mut context = microcad_std::ContextBuilder::new(source_file)
                .with_std()
                .build();

            let node = context.eval().unwrap();
            let mut w = std::io::stdout();
            context.diag().pretty_print(&mut w, &context).unwrap();

            microcad_std::export(node).unwrap();
        }
        Cli { input: None } => {
            eprintln!("No input file specified");
            std::process::exit(1);
        }
    }
}

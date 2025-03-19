// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad command line interpreter

extern crate clap;
extern crate microcad_lang;

use std::{io::Write, path::Path};

use clap::{Parser, Subcommand};
use microcad_lang::{eval::*, objects::*, parse::*, resolve::*};

/// µcad cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Standard library search path
    #[arg(long, default_value = "lib")]
    std: String,

    /// display processing time
    #[arg(short, long, default_value = "false")]
    time: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a µcad file
    Parse {
        /// Input µcad file
        input: String,
        /// print parse tree
        #[clap(short, long)]
        tree: bool,
        /// print formatted code
        #[clap(short, long)]
        fmt: bool,
    },

    /// Parse and resolve a µcad file
    Resolve {
        /// Input µcad file
        input: String,
    },

    /// Parse and evaluate a µcad file
    Eval {
        /// Input µcad file
        input: String,
    },

    /// Parse and evaluate and export a µcad file
    Export {
        /// Input µcad file
        input: String,
    },

    /// Create a new source file with µcad extension
    Create { path: String },
}

/// Main of the command line interpreter
fn main() {
    let cli = Cli::parse();

    let start = std::time::Instant::now();

    if let Err(err) = run(&cli) {
        eprintln!("{err}")
    }

    println!("Processing Time: {:?}", start.elapsed());
}

struct S<'a>(&'a SourceFile);

impl std::fmt::Display for S<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.print_syntax(f, 0)
    }
}

fn run(cli: &Cli) -> anyhow::Result<()> {
    match &cli.command {
        Commands::Parse { input, tree, fmt } => {
            let source_file = parse(input)?;
            if *tree {
                let s = S(&source_file);
                println!("{s}");
            }
            if *fmt {
                println!("{source_file}");
            }
        }
        Commands::Resolve { input } => {
            let symbol_table = resolve(parse(input)?)?;
            println!("{}", symbol_table.borrow());
            eprintln!("Resolved successfully!");
        }
        Commands::Eval { input } => {
            eval(parse(input)?, &cli.std)?;
            eprintln!("Evaluated successfully!");
        }
        /*
        Commands::Export { input } => {
            let exports = export(eval(parse(input)?, &cli.std)?)?;
            eprintln!(
                "Exported {} successfully!",
                exports
                    .iter()
                    .map(|f| f.to_string_lossy().to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }*/
        Commands::Create { path } => {
            use std::{fs::*, path::*};
            let mut path = PathBuf::from(path);
            if path.extension().is_none() {
                path.set_extension("µcad");
            }

            if path.exists() {
                eprintln!("Error: File {path:?} already exists.")
            } else {
                // create demo program
                let mut f = File::create(path.clone())?;
                f.write_all(
                    r#"// µcad generated file
use std::*;

module main() {
  print( "Hello µcad" );
}

main();
"#
                    .as_bytes(),
                )?;
                eprintln!("File {path:?} generated.")
            }
        }
        _ => todo!(),
    }

    Ok(())
}

fn parse(input: impl AsRef<Path>) -> anyhow::Result<SourceFile> {
    Ok(SourceFile::load(input)?)
}
fn resolve(source_file: SourceFile) -> anyhow::Result<SymbolNodeRc> {
    Ok(source_file.resolve(None))
}

fn eval(source_file: SourceFile, _std: impl AsRef<Path>) -> anyhow::Result<ObjectNode> {
    let source_file = std::rc::Rc::new(source_file);
    let mut context = EvalContext::from_source_file(source_file.clone());

    let node = source_file
        .eval(&mut context)
        .map_err(|err| anyhow::anyhow!("{err}"))?;

    Ok(node)
}

/*
fn export(node: ObjectNode) -> anyhow::Result<Vec<std::path::PathBuf>> {
    Ok(microcad_builtin::export(node)?)
}
*/

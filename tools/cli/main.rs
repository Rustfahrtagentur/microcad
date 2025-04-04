// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad command line interpreter

extern crate clap;
extern crate microcad_lang;

use clap::{Parser, Subcommand};
use microcad_lang::{eval::*, objects::*, parse::ParseResult, resolve::*, syntax::*};
use std::{io::Write, rc::Rc};

/// µcad cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
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
        input: std::path::PathBuf,
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
        input: std::path::PathBuf,
        /// print parse tree
        #[clap(short, long)]
        tree: bool,
        /// print formatted code
        #[clap(short, long)]
        fmt: bool,
    },

    /// Parse and evaluate a µcad file
    Eval {
        /// Input µcad file
        input: std::path::PathBuf,
        /// Paths to search for files
        #[arg(short = 'I', long = "input", action = clap::ArgAction::Append)]
        input_path: Vec<std::path::PathBuf>,
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

fn run(cli: &Cli) -> anyhow::Result<()> {
    match &cli.command {
        Commands::Parse { input, tree, fmt } => parse(input, *tree, *fmt)?,
        Commands::Resolve { input, tree, fmt } => resolve(input, *tree, *fmt)?,
        Commands::Eval { input, input_path } => {
            eval(SourceFile::load(input)?, Externals::new(input_path.clone()))?;
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

fn parse(input: impl AsRef<std::path::Path>, tree: bool, fmt: bool) -> ParseResult<()> {
    let source_file = SourceFile::load(input)?;
    println!("Parse Output:\n");
    if tree {
        println!("{}", FormatSyntax(source_file.as_ref()));
    }
    if fmt {
        println!("Parse Output:\n{source_file}");
    }
    eprintln!("Parsed successfully!");
    Ok(())
}

fn resolve(input: impl AsRef<std::path::Path>, tree: bool, fmt: bool) -> ParseResult<()> {
    let source_file = SourceFile::load(input)?;
    eprintln!("Parsed successfully!");

    let symbol_table = source_file.resolve(None);

    println!("Symbols:\n");

    if tree {
        println!("{}", FormatSymbol(&symbol_table.borrow()));
    }
    if fmt {
        println!("{}", symbol_table.borrow());
    }
    eprintln!("Resolved successfully!");
    Ok(())
}

fn eval(source_file: Rc<SourceFile>, externals: Externals) -> anyhow::Result<ObjectNode> {
    let mut context = EvalContext::from_source_file(source_file.clone(), externals);

    let _result = source_file
        .eval(&mut context)
        .map_err(|err| anyhow::anyhow!("{err}"))?;

    /*match result {
        EvalReturn::ObjectNode(node) => Ok(node),
        _ => unreachable!("Return value must be a node"),
    }*/

    eprintln!("Evaluated successfully!");
    todo!();
}

/*
fn export(node: ObjectNode) -> anyhow::Result<Vec<std::path::PathBuf>> {
    Ok(microcad_builtin::export(node)?)
}
*/

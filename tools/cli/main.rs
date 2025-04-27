// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad command line interpreter

extern crate clap;
extern crate microcad_lang;

use clap::{Parser, Subcommand};
use log::*;
use microcad_lang::{
    env_logger_init, eval::*, objects::*, parse::ParseResult, rc::*, resolve::*, syntax::*,
};
use std::io::Write;

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
    },

    /// Parse and resolve a µcad file
    Resolve {
        /// Input µcad file
        input: std::path::PathBuf,
    },

    /// Parse and evaluate a µcad file
    Eval {
        /// Input µcad file
        input: std::path::PathBuf,
        /// Paths to search for files
        #[arg(short = 'I', long = "input", action = clap::ArgAction::Append, default_value = "./lib")]
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
    env_logger_init();

    let cli = Cli::parse();

    let start = std::time::Instant::now();

    if let Err(err) = run(&cli) {
        eprintln!("{err}")
    }

    info!("Processing Time: {:?}", start.elapsed());
}

fn run(cli: &Cli) -> anyhow::Result<()> {
    match &cli.command {
        Commands::Parse { input } => {
            parse(input)?;
        }
        Commands::Resolve { input } => {
            resolve(input)?;
        }
        Commands::Eval { input, input_path } => {
            eval(input, input_path)?;
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

fn parse(input: impl AsRef<std::path::Path>) -> ParseResult<Rc<SourceFile>> {
    let source_file = SourceFile::load(input)?;
    info!("Parsed successfully!");
    Ok(source_file)
}

fn resolve(input: impl AsRef<std::path::Path>) -> ParseResult<SymbolNodeRcMut> {
    let symbol_node = parse(input)?.resolve(None);
    info!("Resolved successfully!");
    Ok(symbol_node)
}

fn eval(
    input: impl AsRef<std::path::Path>,
    search_paths: &[std::path::PathBuf],
) -> anyhow::Result<ObjectNode> {
    let symbols = resolve(input)?;
    let source_file = match symbols.borrow().def.clone() {
        SymbolDefinition::SourceFile(source_file) => source_file,
        _ => todo!(),
    };

    let mut context = EvalContext::new(
        symbols.clone(),
        microcad_builtin::builtin_namespace(),
        search_paths,
        None,
    );
    let result = source_file
        .eval(&mut context)
        .map_err(|err| anyhow::anyhow!("{err}"))?;

    println!("{result}");
    match context.errors_as_str() {
        Some(errors) => {
            warn!("Evaluated with errors:");
            error!("{}", errors);
        }
        None => info!("Evaluated successfully!"),
    }

    todo!("object node output")
}

/*
fn export(node: ObjectNode) -> anyhow::Result<Vec<std::path::PathBuf>> {
    Ok(microcad_builtin::export(node)?)
}
*/

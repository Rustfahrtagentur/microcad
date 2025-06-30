// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad command line interpreter

extern crate clap;
extern crate microcad_lang;

use clap::{Parser, Subcommand};
use microcad_lang::{
    diag::*, eval::*, model_tree::*, parse::*, rc::*, resolve::*, syntax::*, value::Value,
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
        #[arg(short = 'L', long = "lib", action = clap::ArgAction::Append, default_value = "./lib")]
        search_paths: Vec<std::path::PathBuf>,
    },

    /// Parse and evaluate and export a µcad file
    Export {
        /// Input µcad file
        input: String,

        /// Paths to search for files
        #[arg(short = 'L', long = "lib", action = clap::ArgAction::Append, default_value = "./lib")]
        search_paths: Vec<std::path::PathBuf>,

        /// List all export
        #[arg(short)]
        list: bool,
    },

    /// Create a new source file with µcad extension
    Create { path: String },
}

/// Main of the command line interpreter
fn main() {
    env_logger::init();

    let cli = Cli::parse();

    let start = std::time::Instant::now();

    if let Err(err) = run(&cli) {
        eprintln!("{err}")
    }

    log::info!("Processing Time: {:?}", start.elapsed());
}

fn run(cli: &Cli) -> anyhow::Result<()> {
    match &cli.command {
        Commands::Parse { input } => {
            parse(input)?;
        }
        Commands::Resolve { input } => {
            resolve(input)?;
        }
        Commands::Eval {
            input,
            search_paths,
        } => {
            eval(input, search_paths)?;
        }
        Commands::Export {
            input,
            search_paths,
            list,
        } => {
            export(input, search_paths, *list)?;
        }
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

part main() {
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
    log::info!("Parsed successfully!");
    Ok(source_file)
}

fn resolve(input: impl AsRef<std::path::Path>) -> ParseResult<Symbol> {
    let symbol_node = parse(input)?.resolve(None);
    log::info!("Resolved successfully!");
    Ok(symbol_node)
}

fn eval(
    input: impl AsRef<std::path::Path>,
    search_paths: &[std::path::PathBuf],
) -> anyhow::Result<(ModelNode, Context)> {
    let symbols = resolve(input)?;
    let mut context = Context::new(
        symbols.clone(),
        microcad_builtin::builtin_module(),
        search_paths,
        Box::new(Stdout),
    );
    let node = context.eval().map_err(|err| anyhow::anyhow!("{err}"))?;

    log::info!("Result:");
    println!("{node}");
    match context.has_errors() {
        true => {
            log::warn!("Evaluated with errors:");
            eprintln!("{}", context.diagnosis());
        }
        false => log::info!("Evaluated successfully!"),
    }

    Ok((node, context))
}

fn export(
    input: impl AsRef<std::path::Path>,
    search_paths: &[std::path::PathBuf],
    _list_only: bool,
) -> anyhow::Result<Value> {
    let (node, _context) = eval(input, search_paths)?;

    let export_nodes = node
        .source_file_descendants()
        .filter_map(|node| {
            let b = node.borrow();

            if let Some(attributes) = b.attributes().get_as_tuple(&Identifier::no_ref("export")) {
                Some((node.clone(), attributes.clone()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    todo!("Export the nodes by finding a matching exporter from a filename");

    Ok(Value::None)
}

/*
fn export(node: ObjectNode) -> anyhow::Result<Vec<std::path::PathBuf>> {
    Ok(microcad_builtin::export(node)?)
}
*/

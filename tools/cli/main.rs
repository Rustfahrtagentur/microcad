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
    #[arg(short, long, default_value = "false", global = true)]
    time: bool,

    /// Paths to search for files.
    #[arg(short = 'p', long = "search-path", action = clap::ArgAction::Append, default_value = "./lib", global = true)]
    search_paths: Vec<std::path::PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Parse { input } => {
                self.parse(input)?;
            }
            Commands::Resolve { input } => {
                self.resolve(input)?;
            }
            Commands::Eval { input } => {
                self.eval(input)?;
            }
            Commands::Export {
                input,
                output,
                list,
                target,
            } => {
                self.export(input, output, target, *list)?;
            }
            Commands::Create { path } => {
                self.create(path)?;
            }
        }

        Ok(())
    }

    fn parse(&self, input: impl AsRef<std::path::Path>) -> ParseResult<Rc<SourceFile>> {
        let source_file = SourceFile::load(input)?;
        log::info!("Parsed successfully!");
        Ok(source_file)
    }

    fn resolve(&self, input: impl AsRef<std::path::Path>) -> ParseResult<Symbol> {
        let symbol_node = self.parse(input)?.resolve(None);
        log::info!("Resolved successfully!");
        Ok(symbol_node)
    }

    fn eval(&self, input: impl AsRef<std::path::Path>) -> anyhow::Result<(ModelNode, Context)> {
        let mut context =
            microcad_builtin::builtin_context(self.resolve(input)?, &self.search_paths);
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
        &self,
        input: impl AsRef<std::path::Path>,
        _output: &Option<std::path::PathBuf>,
        _target: &Vec<String>,
        _list_only: bool,
    ) -> anyhow::Result<Value> {
        let _ = _target;
        let (_node, _context) = self.eval(input)?;

        Ok(Value::None)
    }

    fn create(&self, path: impl AsRef<std::path::Path>) -> anyhow::Result<()> {
        let mut path = path.as_ref().to_path_buf();
        if path.extension().is_none() {
            path.set_extension("µcad");
        }

        if path.exists() {
            eprintln!("Error: File {path:?} already exists.")
        } else {
            // create demo program
            let mut f = std::fs::File::create(path.clone())?;
            f.write_all(
                r#"// µcad generated file
use std::*;

sketch main() {
  print("Hello µcad");
}

main();
"#
                .as_bytes(),
            )?;
            eprintln!("File {path:?} generated.")
        }

        Ok(())
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Parse a µcad file.
    Parse {
        /// Input µcad file.
        input: std::path::PathBuf,
    },

    /// Parse and resolve a µcad file.
    Resolve {
        /// Input µcad file.
        input: std::path::PathBuf,
    },

    /// Parse and evaluate a µcad file.
    Eval {
        /// Input µcad file.
        input: std::path::PathBuf,
    },

    /// Parse and evaluate and export a µcad file.
    Export {
        /// Input µcad file.
        input: std::path::PathBuf,

        /// Output µcad file.
        output: Option<std::path::PathBuf>,

        /// List all export target files.
        #[arg(short = 'l', long = "list", action = clap::ArgAction::SetTrue)]
        list: bool,

        /// Export a specific target.
        #[arg(short = 't', long = "target", action = clap::ArgAction::Append)]
        target: Vec<String>,
    },

    /// Create a new source file with µcad extension.
    Create { path: std::path::PathBuf },
}

/// Main of the command line interpreter
fn main() {
    env_logger::init();

    let cli: Cli = Parser::parse();

    let start = std::time::Instant::now();

    if let Err(err) = cli.run() {
        eprintln!("{err}")
    }

    log::info!("Processing Time: {:?}", start.elapsed());
}

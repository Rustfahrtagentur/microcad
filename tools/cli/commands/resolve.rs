// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI resolve command.

use microcad_lang::{diag::*, resolve::*};

use crate::*;
use anyhow::*;

#[derive(clap::Parser)]
pub struct Resolve {
    #[clap(flatten)]
    pub parse: Parse,

    /// Print resolve context.
    #[clap(long)]
    pub resolve: bool,

    /// Paths to search for files.
    ///
    /// By default, `./lib` (if it exists) and `~/.microcad/lib` are used.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append)]
    pub search_paths: Vec<std::path::PathBuf>,

    /// Load config from file.
    #[arg(short, long)]
    omit_default_libs: bool,
}

impl RunCommand<ResolveContext> for Resolve {
    fn run(&self, cli: &Cli) -> anyhow::Result<ResolveContext> {
        // run prior parse step
        let root = self.parse.run(cli)?;

        // add default paths or omit this step by option
        let mut search_paths = self.search_paths.clone();

        if !self.omit_default_libs {
            search_paths.append(&mut Cli::default_search_paths())
        };

        // search for a usable std library
        if !search_paths.iter().any(|dir| {
            let file_path = dir.join("std/mod.µcad");
            file_path.exists() && file_path.is_file()
        }) {
            eprintln!(
                "Warning: No std library was found in given search paths: {:?}.
Use `microcad install std` to install the std library.",
                search_paths
            );
        }

        // resolve the file
        let context = ResolveContext::create(
            root,
            &search_paths,
            Some(microcad_builtin::builtin_module()),
            DiagHandler::default(),
        )?;

        if context.has_errors() {
            eprint!("{}", context.diagnosis());
        }

        if self.resolve {
            print!("{context}");
        }

        if cli.is_resolve() {
            eprintln!("Resolved successfully!");
        }

        Ok(context)
    }
}
